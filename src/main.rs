pub mod wave;

#[allow(dead_code)]
static version: &'static str = "rustc 0.11.0-pre-nightly (380657557cb3793d39dfc0d2321fc946cb3496f5 2014-07-02 00:21:36 +0000)";



fn main() {
	println!("{}\n", version);

	wave::read_file_data("../wav/Warrior Concerto - no meta.wav");

	wave::get_audio("../wav/Warrior Concerto - no meta.wav");
}