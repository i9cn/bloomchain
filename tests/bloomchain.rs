extern crate bloomchain;
extern crate rustc_serialize;

mod util;

use bloomchain::{Bloom, BloomChain, Config};
use util::{BloomMemoryDatabase, FromHex, for_each_bloom};

#[test]
fn simple_test_bloom_search() {
	let config = Config::default();
	let mut db = BloomMemoryDatabase::default();
	let bloom = Bloom::from_hex("00000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002020000000000000000000000000000000000000000000008000000001000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");

	let modified_blooms = {
		let chain = BloomChain::new(config, &db);
		let block_number = 23;
		chain.insert(block_number, bloom.clone())
	};

	// number of modified blooms should always be equal number of levels
	assert_eq!(modified_blooms.len(), config.levels);
	db.insert_blooms(modified_blooms);

	{
		let chain = BloomChain::new(config, &db);
		let blocks = chain.with_bloom(&(0..100), &bloom);
		assert_eq!(blocks.len(), 1);
		assert_eq!(blocks[0], 23);
	}

	{
		let chain = BloomChain::new(config, &db);
		let blocks = chain.with_bloom(&(0..22), &bloom);
		assert_eq!(blocks.len(), 0);
	}

	{
		let chain = BloomChain::new(config, &db);
		let blocks = chain.with_bloom(&(23..23), &bloom);
		assert_eq!(blocks.len(), 1);
		assert_eq!(blocks[0], 23);
	}

	{
		let chain = BloomChain::new(config, &db);
		let blocks = chain.with_bloom(&(24..100), &bloom);
		assert_eq!(blocks.len(), 0);
	}
}

#[test]
fn file_test_bloom_search() {
	let config = Config::default();
	let mut db = BloomMemoryDatabase::default();
	let blooms_file = include_bytes!("data/blooms.txt");

	for_each_bloom(blooms_file, | block_number, bloom | {
		let modified_blooms = {
			let chain = BloomChain::new(config, &db);
			chain.insert(block_number, bloom)
		};

		// number of modified blooms should always be equal number of levels
		assert_eq!(modified_blooms.len(), config.levels);
		db.insert_blooms(modified_blooms);
	});

	for_each_bloom(blooms_file, | block_number, bloom | {
		let chain = BloomChain::new(config, &db);
		let blocks = chain.with_bloom(&(block_number..block_number), &bloom);
		assert_eq!(blocks.len(), 1);
		assert_eq!(blocks[0], block_number);
	});
}
