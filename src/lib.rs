use hex_literal::hex;
use sha2::{Digest, Sha256};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, require};

#[derive(
    BorshDeserialize, BorshSerialize, Clone, Copy, near_sdk::serde::Serialize, schemars::JsonSchema,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Stage {
    New,
    LOLisTransferred,
    NFTisTransferred,
    Final,
}

impl Default for Stage {
    fn default() -> Self {
        Self::New
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Quest {
    stage: Stage,
}

#[near_bindgen]
impl Quest {
    pub fn ft_on_transfer(
        &mut self,
        sender_id: near_sdk::AccountId,
        amount: near_sdk::json_types::U128,
        //msg: String,
    ) -> near_sdk::json_types::U128 {
        require!(near_sdk::env::predecessor_account_id().as_ref() == "lolcoin.qbit.near");
        require!(
            matches!(self.stage, Stage::New),
            "This is the first stage of the quest and it is only available once."
        );
        self.stage = Stage::LOLisTransferred;
        near_sdk::Promise::new(sender_id).transfer(64 * near_sdk::ONE_NEAR);
        near_sdk::env::log_str("Congrats! LOL stage is cleared");
        // Return LOL coins
        amount
    }

    pub fn nft_on_transfer(
        &mut self,
        sender_id: near_sdk::AccountId,
        //previous_owner_id: near_sdk::AccountId,
        //token_id: TokenId,
        //msg: String,
    ) -> bool {
        require!(near_sdk::env::predecessor_account_id().as_ref() == "x.paras.near"); // https://paras.id
        require!(
            matches!(self.stage, Stage::LOLisTransferred),
            "This is the first stage of the quest and it is only available once."
        );
        self.stage = Stage::NFTisTransferred;
        near_sdk::Promise::new(sender_id).transfer(64 * near_sdk::ONE_NEAR);
        near_sdk::env::log_str("Congrats! NFT stage is cleared");
        // Return NFT
        true
    }

    pub fn gimme_my_present(&mut self, secret_key: String) {
        require!(
            matches!(self.stage, Stage::NFTisTransferred),
            "This is the second stage of the quest and it is only available once."
        );

        let mut hasher = Sha256::new();
        hasher.update(secret_key.as_bytes());
        let result = hasher.finalize();
        require!(
            result[..]
                == hex!("b1ab1e892617f210425f658cf1d361b5489028c8771b56d845fe1c62c1fbc8b0")[..],
            "The secret key is invalid. Just ask to get the secret key."
        );

        self.stage = Stage::Final;
        near_sdk::Promise::new(near_sdk::env::predecessor_account_id())
            .transfer(128 * near_sdk::ONE_NEAR);
        near_sdk::env::log_str("Happy birthday!");
    }

    pub fn get_stage(&self) -> Stage {
        self.stage
    }
}
