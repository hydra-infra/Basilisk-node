use super::*;
use mock::Call;
use pretty_assertions::assert_eq;

#[test]
fn make_offer_should_work_when_no_nft_exists() {
	//Arrange
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, 200_000 * UNITS), (CHARLIE, 150_000 * UNITS)])
		.build()
		.execute_with(|| {
			// Act
			assert_ok!(Market::make_offer(
				Origin::signed(CHARLIE),
				COLLECTION_ID_0,
				ITEM_ID_0,
				50 * UNITS,
				2
			));

			// Assert
			assert_eq!(
				Market::offers((COLLECTION_ID_0, ITEM_ID_0), CHARLIE),
				Some(Offer {
					maker: CHARLIE,
					amount: 50 * UNITS,
					expires: 2,
				})
			);
			assert_eq!(<Test as Config>::Currency::reserved_balance(&CHARLIE), 50 * UNITS);

			assert_eq!(
				last_event(),
				Event::OfferPlaced {
					who: CHARLIE,
					collection: COLLECTION_ID_0,
					item: ITEM_ID_0,
					amount: 50 * UNITS,
					expires: 2,
				}
				.into()
			);
		});
}

#[test]
fn make_offer_should_fail_when_offer_is_lower_than_minimal_amount() {
	//Arrange
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, 200_000 * UNITS)])
		.build()
		.execute_with(|| {
			// Act & assert
			assert_noop!(
				Market::make_offer(
					Origin::signed(BOB),
					COLLECTION_ID_0,
					ITEM_ID_0,
					<Test as Config>::MinimumOfferAmount::get() - 1,
					1
				),
				Error::<Test>::OfferTooLow
			);
			assert_eq!(Market::offers((COLLECTION_ID_0, ITEM_ID_0), CHARLIE), None);
		});
}

#[test]
fn make_offer_should_fail_when_offer_has_been_already_made() {
	//Arrange
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, 200_000 * UNITS), (BOB, 15_000 * UNITS)])
		.build()
		.execute_with(|| {
			assert_ok!(Market::make_offer(
				Origin::signed(BOB),
				COLLECTION_ID_0,
				ITEM_ID_0,
				50 * UNITS,
				1
			));

			// Act and assert
			assert_noop!(
				Market::make_offer(Origin::signed(BOB), COLLECTION_ID_0, ITEM_ID_0, 70 * UNITS, 1),
				Error::<Test>::AlreadyOffered
			);
		});
}

#[test]
fn make_offer_should_fail_when_offerer_has_not_enough_balance() {
	//Arrange
	let balance = 200_000;
	ExtBuilder::default()
		.with_endowed_accounts(vec![(DAVE, balance * UNITS)])
		.build()
		.execute_with(|| {
			// Act and assert
			let call = Call::Marketplace(crate::Call::<Test>::make_offer {
				collection_id: COLLECTION_ID_0,
				item_id: ITEM_ID_0,
				amount: (balance + 1) * UNITS,
				expires: 2,
			});
			assert_noop!(
				call.dispatch(Origin::signed(DAVE)),
				pallet_balances::Error::<Test, _>::InsufficientBalance
			);
		});
}
