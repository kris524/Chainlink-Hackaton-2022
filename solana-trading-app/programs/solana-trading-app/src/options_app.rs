// // pseudo code

// user {
//     address of better
//     
//    }
   
//    transaction_pay {
//     ammount to pay              
//     token               
//    }
   
//    transaction_buy {
//     ammount to buy             
//     token            
    // experation time
//    }
    // reciever{
        // address you bet against
    // }

#[account]
pub struct User {
    pub address: Pubkey;
}

#[account]
pub struct TransactionPay {
    pub Ammount: f32;
    pub token: ?
}

#[account]
pub TransactionBuy {
    pub Ammount: f32,
    pub token: ?
}

#[account]
pub reciever {
    pub address: Pubkey
}