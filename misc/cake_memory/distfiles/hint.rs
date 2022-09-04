// REDUCTED

static SOUNDS: [SoundName; 24] = [
    SoundName::VoiceBlue, SoundName::VoiceRed,
    SoundName::VoiceYellow, SoundName::VoiceGreen,
    SoundName::VoiceViolet, SoundName::VoiceOrange,
    SoundName::VoiceWhite, SoundName::VoiceJ,
    SoundName::VoiceHeart, SoundName::VoiceQuestion,
    SoundName::VoicePi, SoundName::VoiceSmiley,
    SoundName::VoiceOmega, SoundName::VoiceTurquoise,
    SoundName::VoiceTheta, SoundName::VoiceG,
    SoundName::VoiceKitten, SoundName::VoiceTangerine,
    SoundName::VoiceCake, SoundName::VoiceLambda,
    SoundName::VoiceBurgundy, SoundName::VoiceE,
    SoundName::VoiceCoquelicot, SoundName::VoiceFlag
];

// REDUCTED

struct MusicalMemory {
    // REDUCTED
    mem_order: Vec<usize>,
    mem_sound: Vec<SoundName>,
    // REDUCTED
}

// REDUCTED

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    // REDUCTED
            current = self.sound.get_mut(&self.mem_sound[
                self.mem_order[self.show_count]
            ]);
    // REDUCTED
    }

// REDUCTED
