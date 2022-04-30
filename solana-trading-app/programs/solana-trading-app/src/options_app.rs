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
    pub address: String;
}

#[account]
pub struct TransactionPay {
    pub Ammount: f32;
    pub token: 
}