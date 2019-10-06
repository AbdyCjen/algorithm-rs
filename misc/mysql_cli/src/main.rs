extern crate mysql;

use mysql as my;


fn main() {
    let pool = my::Pool::new("mysql://root:1234@127.0.0.1:3306/bankpay").unwrap();

    pool.prep_exec("select F_order_id, F_trans_id from bankpay0.t_order_09",()).map(|result| {
        result.for_each(|row|{
            let (order_id, trans_id):(String, String) = my::from_row(row.unwrap());
            println!("{}:{}", order_id, trans_id );
        })
    }).unwrap();
}
