import enum
import json
import os
import pyxel
import random
import requests
import time

HOST = os.getenv("HOST", "misc.2022.cakectf.com")
PORT = os.getenv("PORT", 10011)

class GameState(enum.Enum):
    NONE = enum.auto()
    INIT = enum.auto()
    GAME = enum.auto()
    WIN  = enum.auto()
    LOSE = enum.auto()
    DRAW = enum.auto()
    FLAG = enum.auto()

class GameAction(enum.Enum):
    HIT = enum.auto()
    STAND = enum.auto()
    SKIP = enum.auto()

class ServerConnection(object):
    def __init__(self, host, port):
        self.host = host
        self.port = port
        self.session = None

    def request(self, endpoint, data=None):
        try:
            r = requests.get(f"http://{self.host}:{self.port}/{endpoint}",
                             params=data,
                             cookies=self.session)
        except requests.exceptions.RequestException as e:
            print("[ERROR]", e)
            print(f"Server '{self.host}:{self.port}' is down...?")
            exit(1)

        if self.session is None:
            self.session = r.cookies
        return json.loads(r.text)

class Game(object):
    def __init__(self):
        self.reset()
        pyxel.init(168, 128, title='ALLBET BLACKJACK', display_scale=3)
        pyxel.load("../distfiles/client/resource.pyxres")
        #pyxel.playm(0, loop=True)
        pyxel.run(self.update, self.draw)

    def reset(self):
        self.state = GameState.INIT
        self.frame = 0
        self.player_score = 0
        self.dealer_score = 0
        self.player_hand = []
        self.dealer_hand = []
        self.num_dealer_cards = 0
        self.connection = ServerConnection(HOST, PORT)
        r = self.connection.request('user/new')
        self.user_id = r['user_id']
        self.money = r['money']
        self.title_cards = random.sample(
            [(i // 13, i % 13) for i in range(4*13)], 5
        )

    def update(self):
        """Update game state and handle key events"""
        if self.state == GameState.INIT:
            if pyxel.btnp(pyxel.KEY_SPACE): # Game start
                pyxel.play(3, [1])
                self.frame = 0
                self.init_game()
                self.state = GameState.GAME

        elif self.state == GameState.GAME:
            if pyxel.btnp(pyxel.KEY_Z): # Player chose hit
                pyxel.play(3, [0])
                self.notify_action(GameAction.HIT)

            elif pyxel.btnp(pyxel.KEY_X): # Player chose stand
                pyxel.play(3, [0])
                self.notify_action(GameAction.STAND)

            elif pyxel.btnp(pyxel.KEY_C): # Player chose skip
                pyxel.play(3, [0])
                self.notify_action(GameAction.SKIP)

        elif self.state == GameState.DRAW or self.state == GameState.WIN:
            if pyxel.btnp(pyxel.KEY_Z):
                pyxel.play(3, [0])
                self.init_game()
                self.state = GameState.GAME
                self.frame = 0

        elif self.state == GameState.LOSE:
            if pyxel.btnp(pyxel.KEY_Z):
                pyxel.play(3, [0])
                self.reset()
                self.init_game()
                self.state = GameState.GAME

        self.frame += 1

    def draw(self):
        """Draw game screen"""
        pyxel.cls(0)
        pyxel.rect(0, 0, 256, 192, 11)

        if self.state == GameState.INIT:
            # Draw title screen
            pyxel.text(72, 16, "ALL-BET", 2)
            pyxel.text(68, 24, "B A K A K", 0)
            pyxel.text(68, 27, " L C J C ", 0)
            if (self.frame // 3) % 10 < 5:
                pyxel.text(48, 108, "PRESS SPACE TO START", 8)

            self.draw_cards(self.title_cards, 40, 48)

        else:
            # Draw status
            pyxel.text(8,  8, "DEALER", 0)
            pyxel.text(8,  92, "PLAYER", 0)
            pyxel.text(8, 100, f"BET: ${self.money}", 0)
            if self.player_score != -1:
                pyxel.text(8, 108, f"TOTAL: {self.player_score}", 0)
            else:
                pyxel.text(8, 108, "BURST!", 0)

            self.draw_cards(self.deck[-2:], 4, 28)
            pyxel.text(8, 80, "NEXT CARDS", 8)

            self.draw_cards(self.player_hand, 56, 60)
            self.draw_cards(self.dealer_hand, 56, 4)
            if self.dealer_score != -1:
                pyxel.text(8, 16, f"TOTAL: {self.dealer_score}", 0)
            else:
                pyxel.text(8, 16, "BURST!", 0)

            if self.state == GameState.GAME:
                if (self.frame // 3) % 10 < 8:
                    pyxel.text(24, 116, "[Z] HIT / [X] STAND / [C] SKIP", 8)

            else:
                if (self.frame // 3) % 10 < 8:
                    if self.state == GameState.DRAW:
                        pyxel.text(28, 116, "DRAW! PRESS [Z] TO CONTINUE", 8)
                    elif self.state == GameState.LOSE:
                        pyxel.text(24, 116, "YOU LOSE! PRESS [Z] TO RESTART", 8)
                    elif self.state == GameState.WIN:
                        pyxel.text(24, 116, "YOU WIN! PRESS [Z] TO CONTINUE", 8)
                    elif self.state == GameState.FLAG:
                        pyxel.text(28, 116, self.flag, 8)

    def draw_cards(self, cards, x_origin, y_origin):
        """Draw cards"""
        if isinstance(cards, int):
            for i in range(cards):
                pyxel.blt(x_origin+i*16, y_origin, 1, 0, 144, 32, 48, 5)
        else:
            for i, (t, c) in enumerate(cards):
                if c < 5:
                    pyxel.blt(x_origin+i*16, y_origin, 0,
                              32*t, 48*c, 32, 48, 5)
                elif c < 10:
                    pyxel.blt(x_origin+i*16, y_origin, 0,
                              32*(4+t), 48*(c-5), 32, 48, 5)
                else:
                    pyxel.blt(x_origin+i*16, y_origin, 1,
                              32*t, 48*(c-10), 32, 48, 5)

    def notify_action(self, act):
        """Player chose hit or stand"""
        if act == GameAction.HIT:
            # Hit
            response = self.connection.request('game/act', {'action': 'hit'})
            self.deck.pop()
            if response['num_dealer_cards'] != len(self.dealer_hand):
                self.dealer_hand.append(self.deck.pop())

        elif act == GameAction.STAND:
            # Stand
            response = self.connection.request('game/act', {'action': 'stand'})

        elif act == GameAction.SKIP:
            # Skip
            response = self.connection.request('game/act', {'action': 'xxx'})
            next_card = self.deck.pop()
            while response['num_dealer_cards'] != len(self.dealer_hand):
                self.dealer_hand.append(next_card)
                next_card = self.deck.pop()

        self.player_score = response['player_score']
        self.player_hand = response['player_hand']
        self.num_dealer_cards = response['num_dealer_cards']
        self.dealer_score = calculate_score(self.dealer_hand)
        self.money = response['money']

        # Update state
        if response['state'] == 'win':
            pyxel.play(3, [3])
            self.state = GameState.WIN
        elif response['state'] == 'lose':
            pyxel.play(3, [2])
            self.state = GameState.LOSE
        elif response['state'] == 'draw':
            pyxel.play(3, [3])
            self.state = GameState.DRAW
        elif response['state'] == 'flag':
            pyxel.play(3, [3])
            self.flag = response['flag']
            self.state = GameState.FLAG

    def init_game(self):
        """Start game"""
        response = self.connection.request('game/new')
        for seed in range(int(time.time()), int(time.time()) - 5, -1):
            self.deck = [(i // 13, i % 13) for i in range(4*13)]
            random.seed(seed ^ self.user_id)
            random.shuffle(self.deck)
            if self.deck[-1] == tuple(response['player_hand'][0]) and \
               self.deck[-3] == tuple(response['player_hand'][1]):
                print("Found seed:", seed)
                break
        else:
            print("Bad luck!")
            exit(1)

        self.player_hand = []
        self.dealer_hand = []
        for i in range(2):
            self.player_hand.append(self.deck.pop())
            self.dealer_hand.append(self.deck.pop())
        self.player_score = calculate_score(self.player_hand)
        self.dealer_score = calculate_score(self.dealer_hand)

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

if __name__ == '__main__':
    Game()
