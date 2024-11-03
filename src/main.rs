use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone)]
struct Note {
    title: String,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    tags: Vec<String>,
}

impl Note {
    fn new(title: String, content: String, tags: Vec<String>) -> Note {
        let now = Utc::now();
        Note {
            title,
            content,
            created_at: now,
            updated_at: now,
            tags,
        }
    }

    fn update(&mut self, new_content: String) {
        self.content = new_content;
        self.updated_at = Utc::now();
    }
}

struct NotebookApp {
    notes: Vec<Note>,
    new_title: String,
    new_content: String,
    new_tags: String,
    edit_index: Option<usize>,
}

impl NotebookApp {
    fn load_notes() -> Vec<Note> {
        let path = "notes.json";
        if Path::new(path).exists() {
            let data = fs::read_to_string(path).expect("Konnte Datei nicht laden!");
            serde_json::from_str(&data).expect("Konnte JSON nicht lesen!")
        } else {
            Vec::new()
        }
    }

    fn save_notes(&self) {
        let data = serde_json::to_string(&self.notes).expect("Fehler beim serialisieren!");
        fs::write("notes.json", data).expect("Fehler beim Schreiben!");
    }

    fn add_note(&mut self) {
        let tags: Vec<String> = self.new_tags.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

        let new_note = Note::new(self.new_title.clone(), self.new_content.clone(), tags);
        self.notes.push(new_note);
        self.new_title.clear();
        self.new_content.clear();
        self.new_tags.clear();
        self.save_notes();
    }

    fn delete_note_by_index(&mut self, index: usize) {
        if index < self.notes.len() {
            self.notes.remove(index);
            self.save_notes();
        }
    }

    fn update_note_by_index(&mut self, index: usize, new_content: String) {
        if let Some(note) = self.notes.get_mut(index) {
            note.update(new_content);
            self.save_notes();
        }
    }
}

impl eframe::App for NotebookApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Notizbuch");

            ui.vertical(|ui| {
                eframe::egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                    let mut indices_to_delete = Vec::new();

                    for (index, note) in self.notes.iter().enumerate() {
                        ui.group(|ui| {
                            ui.label(format!("Titel: {}", note.title));
                            ui.label(format!("Inhalt: {}", note.content));
                            ui.label(format!("Erstellt: {}", note.created_at));
                            ui.label(format!("Letzte Änderung: {}", note.updated_at));
                            ui.label(format!("Tags: {:?}", note.tags));
                            
                            if ui.button("Bearbeiten").clicked() {
                                self.new_title = note.title.clone();
                                self.new_content = note.content.clone();
                                self.new_tags = note.tags.join(", ");
                                self.edit_index = Some(index);
                            }

                            if ui.button("Löschen").clicked() {
                                indices_to_delete.push(index);
                            }
                        });
                    }

                    for &index in indices_to_delete.iter().rev() {
                        self.delete_note_by_index(index);
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Titel:");
                    ui.text_edit_singleline(&mut self.new_title);
                });

                ui.horizontal(|ui| {
                    ui.label("Inhalt:");
                    ui.text_edit_multiline(&mut self.new_content);
                });

                ui.horizontal(|ui| {
                    ui.label("Tags (Komma getrennt):");
                    ui.text_edit_singleline(&mut self.new_tags);
                });

                if let Some(edit_index) = self.edit_index {
                    if ui.button("Änderungen speichern").clicked() {
                        if !self.new_content.is_empty() {
                            let tags: Vec<String> = self.new_tags.split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();

                            self.notes[edit_index].title = self.new_title.clone();
                            self.notes[edit_index].update(self.new_content.clone());
                            self.notes[edit_index].tags = tags;
                            self.save_notes();

                            self.new_title.clear();
                            self.new_content.clear();
                            self.new_tags.clear();
                            self.edit_index = None;
                        }
                    }
                } else if ui.button("Neue Notiz hinzufügen").clicked() {
                    if !self.new_title.is_empty() && !self.new_content.is_empty() {
                        self.add_note();
                    }
                }
            });
        });
    }
}

fn main() {
    let app = NotebookApp {
        notes: NotebookApp::load_notes(),
        new_title: String::new(),
        new_content: String::new(),
        new_tags: String::new(),
        edit_index: None,
    };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Notizbuch",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    ).expect("Fehler beim Starten!");
}