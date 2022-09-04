import enum
from flask import abort, Flask, jsonify, session, request
from flask_session import Session
import os
import random
import redis
import time

HOST = os.getenv("HOST", "redis")
PORT = os.getenv("PORT", 6379)

app = Flask(__name__)
# Use redis as session store to prevent session reuse
SESSION_TYPE = 'redis'
SESSION_REDIS = redis.Redis(HOST, PORT, db=0)
app.config.from_object(__name__)
Session(app)

class GameState(enum.Enum):
    IN_PROGRESS  = enum.auto()
    PLAYER_WIN   = enum.auto()
    PLAYER_LOSE  = enum.auto()
    PLAYER_DRAW  = enum.auto()
    PLAYER_CHEAT = enum.auto()

def calculate_score(cards):
    """Calculate current total of cards"""
    num_ace = 0
    score = 0
    for _, c in cards:
        if c == 0: num_ace += 1
        elif c < 10: score += c + 1
        else: score += 10

    while num_ace > 0:
        if 21 - score >= 10 + num_ace: score += 11
        else: score += 1
        num_ace -= 1

    return -1 if score > 21 else score


@app.before_request
def before_request():
    if 'state' in session and session['state'] == GameState.PLAYER_CHEAT:
        abort(400, "Cheat detected")


@app.route('/user/new')
def user_new():
    """Create a new session"""
    session['money'] = 100
    session['state'] = GameState.PLAYER_DRAW
    session['user_id'] = random.randrange(0, 1000000000)
    return jsonify({'money': session['money'], 'user_id': session['user_id']})

@app.route('/game/new')
def game_new():
    """Create a new game"""
    if session['state'] == GameState.IN_PROGRESS:
        # Player tried to abort game
        session['state'] = GameState.PLAYER_CHEAT
        abort(400, "Cheat detected")

    # Shuffle cards
    deck = [(i // 13, i % 13) for i in range(4*13)]
    random.seed(int(time.time()) ^ session['user_id'])
    random.shuffle(deck)
    session['deck'] = deck

    # Create initial hand
    session['player_hand'] = []
    session['dealer_hand'] = []
    for i in range(2):
        session['player_hand'].append(session['deck'].pop())
        session['dealer_hand'].append(session['deck'].pop())
    session['player_score'] = calculate_score(session['player_hand'])
    session['dealer_score'] = calculate_score(session['dealer_hand'])

    # Return state
    session['state'] = GameState.IN_PROGRESS
    return jsonify({
        'player_hand': session['player_hand'],
        'player_score': session['player_score'],
        'num_dealer_cards': len(session['dealer_hand'])
    })

@app.route('/game/act')
def game_act():
    """Player action"""
    if session['state'] != GameState.IN_PROGRESS:
        # Player tried to use expired game
        session['state'] = GameState.PLAYER_CHEAT
        abort(400, "Cheat detected")

    # Draw cards
    player_action = request.args.get('action', 'hit')
    dealer_action = 'stand'
    if player_action == 'hit':
        # Player chose hit (Game continues)
        session['player_hand'].append(session['deck'].pop())
        session['player_score'] = calculate_score(session['player_hand'])

        # Dealer action
        if session['dealer_score'] <= 16:
            session['dealer_hand'].append(session['deck'].pop())
            session['dealer_score'] = calculate_score(session['dealer_hand'])
            dealer_action = 'hit'

    else:
        # Player chose stand (Game ends)
        next_card = session['deck'].pop()
        while session['dealer_score'] <= 16:
            session['dealer_hand'].append(next_card)
            session['dealer_score'] = calculate_score(session['dealer_hand'])
            if session['dealer_score'] == -1:
                # Dealer bursts
                break
            else:
                next_card = session['deck'].pop()

    # Update state
    dealer_hand = session['dealer_hand']
    dealer_score = session['dealer_score']

    if session['player_score'] == session['dealer_score'] == -1 or \
       (player_action == 'stand' and session['player_score'] == session['dealer_score']):
        # Draw
        session['state'] = GameState.PLAYER_DRAW
        state = 'draw'

    elif session['player_score'] == -1 or \
         (player_action == 'stand' and session['player_score'] < session['dealer_score']):
        # Player loses
        session['state'] = GameState.PLAYER_LOSE
        session['money'] = 0
        state = 'lose'

    elif session['dealer_score'] == -1 or \
         (player_action == 'stand' and session['player_score'] > session['dealer_score']):
        # Player wins
        session['state'] = GameState.PLAYER_WIN
        session['money'] *= 2
        state = 'win'

    else:
        # Game in progress
        dealer_hand = None
        dealer_score = 0
        state = 'game'

    # Special bonus
    if session['money'] > 999999999999999:
        flag = os.getenv('FLAG', "CakeCTF{**** REDUCTED ****}")
        state = 'flag'
    else:
        flag = ''

    return jsonify({
        'state': state,
        'player_hand': session['player_hand'],
        'player_score': session['player_score'],
        'dealer_hand': dealer_hand,
        'dealer_score': dealer_score,
        'num_dealer_cards': len(session['dealer_hand']),
        'dealer_action': dealer_action,
        'money': session['money'],
        'flag': flag
    })

if __name__ == '__main__':
    app.run()
