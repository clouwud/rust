fn main() {
    // println!("Hello, world!");

    // let x = -972;
    
    // integer
    // **********************************************************************
    // i8 = -2^7 - 2^7 -1 ( -128 <==> 127 )     u8 = 0 -2^8 -1 ( 0 <==> 255 )        
    // i16      u16
    // i32      u32
    // i64      u64
    // i128     u128
    // i = interger(can use negative) , u = unsigned integer (cant use negative)

    // float
    // f32 f64
    // let floating_point: f32 = 10.9;

    // boolean //true = 1 ; false = 0;
    // let true_or_false; bool = 0;  

    //character
    // let letter: char = 'a';    

    // tuple
    // let tup: (i32, bool, char) = ( 1, true, 's');
    // tup.0 = 10;   
    // println("{}", tup.0);
    
    // array
    let mut arr: [i32 ; 5] = [1,2,3,4,5];  
    arr[4] = 3;
    println!("{}", arr[4]);    

}