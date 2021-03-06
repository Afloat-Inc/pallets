use crate::mock::*;
use frame_support::assert_ok;
use sp_runtime::Permill;

pub struct ExtBuilder;

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {}
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		pallet_balances::GenesisConfig::<Test> {
			balances: vec![(1, 100), (2, 20), (3, 30), (4, 40), (5, 50), (6, 60)],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

#[test]
fn create_frunique_works() {
	// Create a frunique
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(Fruniques::create(Origin::signed(1), 1, 0,Some(Permill::from_percent(50)) ,1));
	});
}

#[test]
fn spawn_extrinsic_works() {
	ExtBuilder::default().build().execute_with(|| {
		//assert_ok!(Fruniques::create(Origin::signed(1), 1,255,Some(100), 1));
		assert_ok!(Fruniques::spawn(Origin::signed(1), 1, 255,true,Permill::from_float(20.525),1) );
		//Fruniques::spawn(Origin::signed(1),1,255,true,Permill::from_float(20.525),1 );
		assert_ok!(Fruniques::spawn(Origin::signed(1), 1, 1, true,Permill::from_float(20.525),1));
		assert_ok!(Fruniques::instance_exists(Origin::signed(1), 1, 1));
	});
}
