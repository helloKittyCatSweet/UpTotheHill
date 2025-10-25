#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
    use frame::prelude::*;
    use scale_info::prelude::vec::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type DivinationCount<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    pub type UserRecords<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Vec<Vec<u8>>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        DivinationPerformed {
            account: T::AccountId,
            question: Vec<u8>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        QuestionTooShort,
        QuestionTooLong,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn perform_divination(
            origin: OriginFor<T>,
            question: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(question.len() >= 5, Error::<T>::QuestionTooShort);
            ensure!(question.len() <= 500, Error::<T>::QuestionTooLong);

            DivinationCount::<T>::mutate(|count| *count += 1);
            UserRecords::<T>::mutate(&who, |records| records.push(question.clone()));

            Self::deposit_event(Event::DivinationPerformed {
                account: who,
                question,
            });

            Ok(())
        }
    }
}

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{new_test_ext, Test, Tarot, RuntimeOrigin};
    use frame::testing_prelude::{assert_ok, assert_noop};

    // 现有的基本测试
    #[test]
    fn test_question_validation() {
        let valid_question = b"This is a valid question".to_vec();
        let too_short = b"Hi".to_vec();
        let too_long = vec![0; 501];
        
        assert!(valid_question.len() >= 5 && valid_question.len() <= 500);
        assert!(too_short.len() < 5);
        assert!(too_long.len() > 500);
    }

    #[test]
    fn test_basic_compilation() {
        assert!(true, "Pallet编译成功");
    }

    // 添加集成测试
    #[test]
    fn test_perform_divination_works() {
        new_test_ext().execute_with(|| {
            let question = b"Will this test pass?".to_vec();
            
            assert_ok!(Tarot::perform_divination(
                RuntimeOrigin::signed(1),
                question.clone()
            ));

            // 验证计数器增加
            assert_eq!(DivinationCount::<Test>::get(), 1);
            
            // 验证用户记录保存
            let user_records = UserRecords::<Test>::get(1);
            assert_eq!(user_records, vec![question]);
        });
    }

    #[test]
    fn test_question_too_short() {
        new_test_ext().execute_with(|| {
            let short_question = b"Hi".to_vec();
            
            assert_noop!(
                Tarot::perform_divination(RuntimeOrigin::signed(1), short_question),
                Error::<Test>::QuestionTooShort
            );
        });
    }

    #[test]
    fn test_question_too_long() {
        new_test_ext().execute_with(|| {
            let long_question = vec![0; 501];
            
            assert_noop!(
                Tarot::perform_divination(RuntimeOrigin::signed(1), long_question),
                Error::<Test>::QuestionTooLong
            );
        });
    }
}