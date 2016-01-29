use std::marker::PhantomData; // use phantomdata to constrain the types of our expressions
use std::fmt;

fn main()
{
	let tinyexpr =  lt(add(var("a"), val(1)), add(var("b"), val(120)));

	// this shouldn't typecheck:
	// let brokenexpr = add(lt(var("a"), var("b")), val(true));

	println!("Tiny expression : {}", tinyexpr.print());

}
// the overall TLExpr typeclass/trait
pub trait TLExpr<T>  {
	fn print(&self) -> String;
	fn t(&self) -> PhantomData<T>;
}
// specialised expressions to numeric or boolean expressions
pub trait TLNumExpr : TLExpr<i32> {}
pub trait TLBoolExpr : TLExpr<bool> {}

/*
	Variables in the tiny language syntax
*/
// declaration
pub struct TLVar<'a, T> {
	var : &'a str,
	var_t : PhantomData<T>
}
// constructor
impl<'a, T> TLVar<'a, T> {
	pub fn new (var: &'a str) -> TLVar<'a, T> {
		TLVar { var : var, var_t : PhantomData } //, var_t: PhantomData }
	}
}
// nice alias for shallow embedding
pub fn var<'a, T> (var: &'a str) -> TLVar<'a, T> {
	return TLVar::new(var);
}
// implementation of the TLExpr trait
impl<'a, T> TLExpr<T> for TLVar<'a, T> {
    fn print(&self) -> String {
    	return self.var.to_string();
    }
    fn t(&self) -> PhantomData<T> {
    	return PhantomData;
    }
}

// 	Constants in the tiny language syntax 

// // declaration
pub struct TLVal<T> where T : fmt::Display { 
	val : T,
	val_t : PhantomData<T>
}
// constructor
impl<T> TLVal<T> where T : fmt::Display {
	pub fn new (val : T) -> TLVal<T> {
		TLVal {val : val, val_t : PhantomData}
	}
}
// nice alias for shallow embedding
pub fn val<T> (val: T) -> TLVal<T> where T : fmt::Display {
	return TLVal::new(val);
}
// implementation of the TLExpr trait
impl<T> TLExpr<T> for TLVal<T> where T : fmt::Display {
	fn print(&self) -> String {
		return format!("{}",self.val);
	}
	fn t(&self) -> PhantomData<T> {
		return self.val_t;
	}
}

// arithmetic functions
// TODO: make this generic!
// addition function declaration
pub struct TLAdd<L,R> where 
	L : TLExpr<i32>, 
	R : TLExpr<i32> 
{
	lhs : L,
	rhs : R
}
// constructor
impl<L: TLExpr<i32>,R: TLExpr<i32>> TLAdd<L,R> {
	pub fn new (lhs : L, rhs : R) -> TLAdd<L,R> {
		TLAdd {lhs : lhs, rhs : rhs}
	}
}
// nice alias for shallow embedding
pub fn add<L : TLExpr<i32>,R : TLExpr<i32>>(lhs : L, rhs : R) -> TLAdd<L,R> {
	return TLAdd::new(lhs,rhs);
}
// implementation of the TLExpr trait
impl<L : TLExpr<i32>, R : TLExpr<i32>> TLExpr<i32> for TLAdd<L,R> {
	fn print(&self) -> String {
		return format!("({}+{})", self.lhs.print(), self.rhs.print());
	}
	fn t(&self) -> PhantomData<i32> {
		return PhantomData;
	} 
}

// boolean functions
// TODO: make this generic!
// less than declaration
pub struct TLLessThan<L,R> where
	L : TLExpr<i32>,
	R : TLExpr<i32>
{
	lhs : L,
	rhs : R
}
// constructor 
impl<L: TLExpr<i32>, R: TLExpr<i32>> TLLessThan<L,R> {
	pub fn new (lhs : L, rhs : R) -> TLLessThan<L,R> {
		TLLessThan {lhs : lhs, rhs: rhs}
	}
}
// nice alias
pub fn lt<L : TLExpr<i32>, R :TLExpr<i32>> (lhs: L, rhs : R) -> TLLessThan<L,R> {
	return TLLessThan::new(lhs,rhs);
}
// implementation of the TLExpr trait
impl<L : TLExpr<i32>, R : TLExpr<i32>> TLExpr<bool> for TLLessThan<L,R> {
	fn print(&self) -> String {
		return format!("({}<{})", self.lhs.print(), self.rhs.print());
	}
	fn t(&self) -> PhantomData<bool> {
		return PhantomData
	}
}