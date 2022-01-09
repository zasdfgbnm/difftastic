#![no_main]
use libfuzzer_sys::fuzz_target;
use typed_arena::Arena;
use difftastic::tree_sitter_parser as tsp;

fuzz_target!(|data: &[u8]| {
    let ts_lang = tsp::from_language(difftastic::guess_language::Language::EmacsLisp);
    let arena = Arena::new();
    let src = String::from_utf8_lossy(&data).to_string();
    let ast = tsp::parse(&arena, &src, &ts_lang);
});
