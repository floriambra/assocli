pub(crate) mod model {

    pub mod dto {
        use serde::{Deserialize, Serialize};
        use validator::Validate;

        #[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
        pub struct GENERIC {
            #[validate(range(max = 100))]
            pub id: i32,
            #[validate(length(min = 1, max = 20))]
            pub name: String,
            pub state: String,
        }

        #[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
        pub struct CreateUpdateGENERIC {
            #[validate(range(max = 100))]
            pub id: Option<i32>,
            #[validate(length(min = 1, max = 20))]
            pub name: Option<String>,
            pub state: Option<String>,
        }
    }
}
