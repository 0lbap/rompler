use rodio::{Decoder, Source};
use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

pub struct App {
    pub char_to_note: HashMap<char, String>,
    pub shared_notes_pressed: Arc<Mutex<HashMap<String, bool>>>,
    pub note_map: HashMap<String, f64>,
    pub source: rodio::source::Buffered<Decoder<BufReader<File>>>,
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

        // Setup the audio file
        let file = BufReader::new(File::open("sounds/piano.mp3").unwrap()); // TODO: add more sounds to the rompler (change with arrow keys?)
        let source = Decoder::new(file).unwrap().buffered();
        App {
            shared_notes_pressed: shared_notes_pressed,
            char_to_note: char_to_note,
            note_map: note_map,
            source: source,
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
}
