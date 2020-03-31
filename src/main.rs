/// Defers evaluation of a block of code until the end of the scope.
// #[cfg(feature="default")]
// #[doc(hidden)]
// macro_rules! defer {
//     ($($body: stmt;)+) => {
//         let _guard = {
//             pub struct Guard<F: FnOnce()>(Option<F>);

//             impl<F: FnOnce()> Drop for Guard<F> {
//                 fn drop(&mut self) {
//                     (self.0).take().map(|f| f())
//                 }
//             }
//             Guard(Some(|| {
//                 let _ = {$($body)* };
//             }))
//         };
//     };
// }
// #[cfg(feature="default")]
// #[doc(hidden)]
macro_rules! defer {
    ($($body: stmt;)+) => {
        let _guard = {
            pub struct Guard<F: FnOnce()>(Option<F>);

            impl<F: FnOnce()> Drop for Guard<F> {
                fn drop(&mut self) {
                    if let Some(f) = self.0.take() {
                        f()
                    }
                }
            }
            Guard(Some(|| {
                $($body)+
            }))
        };
    };
}

fn main() {
   pub struct Guard<F: FnOnce()>(Option<F>);
   impl<F: FnOnce()> Drop for Guard<F> {
       fn drop(&mut self) {
           if let Some(f) = (self.0).take() {
               f()
           }
       }
   } 

   let _guard1 = Guard(Some(|| {
       println!("guard: 1");
   }));
   let _guard2 = Guard(Some(|| {
       println!("guard: 2");
   }));

   defer! {
       println!("guard: 3");
   }

   println!("hello");
}