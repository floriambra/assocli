pub(crate) mod model {

    pub mod dto {
        use serde::{Deserialize, Serialize};
        use validator::Validate;

        #[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
        pub struct GENERIC {
            #[validate(range(max = 100))]
            pub id: i32,
        }
    }
}
