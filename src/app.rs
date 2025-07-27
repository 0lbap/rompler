use rodio::{Decoder, Source, source::Buffered};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

pub struct Instrument {
    pub name: String,
    pub source: Buffered<Decoder<BufReader<File>>>,
}

pub struct App {
    pub char_to_note: HashMap<char, String>,
    pub shared_notes_pressed: Arc<Mutex<HashMap<String, bool>>>,
    pub note_map: HashMap<String, f64>,
    pub instruments: Vec<Instrument>,
    pub current_instrument_index: usize,
}

impl App {
    pub fn new() -> App {
        // Map keyboard keys to music notes
        let mut char_to_note = HashMap::new();
        char_to_note.insert('q', "c".to_string());
        char_to_note.insert('z', "c#".to_string());
        char_to_note.insert('s', "d".to_string());
        char_to_note.insert('e', "d#".to_string());
        char_to_note.insert('d', "e".to_string());
        char_to_note.insert('f', "f".to_string());
        char_to_note.insert('t', "f#".to_string());
        char_to_note.insert('g', "g".to_string());
        char_to_note.insert('y', "g#".to_string());
        char_to_note.insert('h', "a".to_string());
        char_to_note.insert('u', "a#".to_string());
        char_to_note.insert('j', "b".to_string());

        // Initial state of music notes
        let mut notes_pressed = HashMap::new();
        notes_pressed.insert("c".to_string(), false);
        notes_pressed.insert("c#".to_string(), false);
        notes_pressed.insert("d".to_string(), false);
        notes_pressed.insert("d#".to_string(), false);
        notes_pressed.insert("e".to_string(), false);
        notes_pressed.insert("f".to_string(), false);
        notes_pressed.insert("f#".to_string(), false);
        notes_pressed.insert("g".to_string(), false);
        notes_pressed.insert("g#".to_string(), false);
        notes_pressed.insert("a".to_string(), false);
        notes_pressed.insert("a#".to_string(), false);
        notes_pressed.insert("b".to_string(), false);

        let shared_notes_pressed = Arc::new(Mutex::new(notes_pressed));

        // Define the pitch ratios for the notes based on the chromatic scale starting from C
        let mut note_map = HashMap::new();
        note_map.insert("c".to_string(), 1.0);
        note_map.insert("c#".to_string(), 17.0 / 16.0);
        note_map.insert("d".to_string(), 9.0 / 8.0);
        note_map.insert("d#".to_string(), 6.0 / 5.0);
        note_map.insert("e".to_string(), 5.0 / 4.0);
        note_map.insert("f".to_string(), 4.0 / 3.0);
        note_map.insert("f#".to_string(), 45.0 / 32.0);
        note_map.insert("g".to_string(), 3.0 / 2.0);
        note_map.insert("g#".to_string(), 8.0 / 5.0);
        note_map.insert("a".to_string(), 5.0 / 3.0);
        note_map.insert("a#".to_string(), 7.0 / 4.0);
        note_map.insert("b".to_string(), 15.0 / 8.0);

        // Setup audio files for instruments
        let mut instruments = Vec::new();

        // Search for audio files in the sound bank directory and create an Instrument instance for each of them
        let sound_bank_path = "sounds/";
        for entry in fs::read_dir(sound_bank_path).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_file() {
                let entry_path = entry.path();
                let extension = entry_path.extension().unwrap().to_str().unwrap();
                if extension == "mp3" || extension == "wav" {
                    let instrument_name = entry.file_name().into_string().unwrap();
                    let sound_file = BufReader::new(File::open(entry.path()).unwrap());
                    let sound_source = Decoder::new(sound_file).unwrap().buffered();
                    let instrument = Instrument {
                        name: instrument_name,
                        source: sound_source,
                    };
                    instruments.push(instrument);
                }
            }
        }

        if instruments.len() == 0 {
            panic!("No instrument found in the sound bank.")
        }

        App {
            shared_notes_pressed: shared_notes_pressed,
            char_to_note: char_to_note,
            note_map: note_map,
            instruments: instruments,
            current_instrument_index: 0,
        }
    }

    pub fn press_note(&mut self, note: String) {
        self.release_note_after_delay(note.clone(), Duration::from_millis(100));
        let mut notes_pressed = self.shared_notes_pressed.lock().unwrap();
        notes_pressed.insert(note.to_string(), true);
    }

    pub fn release_note_after_delay(&mut self, note: String, dur: Duration) {
        let shared_notes_pressed = self.shared_notes_pressed.clone();

        thread::spawn(move || {
            sleep(dur);
            let mut notes_pressed = shared_notes_pressed.lock().unwrap();
            notes_pressed.insert(note.to_string(), false);
        });
    }

    pub fn next_instrument(&mut self) {
        self.current_instrument_index =
            (self.current_instrument_index + 1) % self.instruments.len();
    }

    pub fn prev_instrument(&mut self) {
        self.current_instrument_index =
            (self.current_instrument_index + self.instruments.len() - 1) % self.instruments.len();
    }
}
