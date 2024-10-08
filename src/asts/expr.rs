
struct Binary {
	left:Expr,
	operator:Token,
	right:Expr,
}
impl Binary {
	pub fn new(
		left:Expr,
		operator:Token,
		right:Expr,
	) -> Binary {
		Binary {
			Expr,
			operator,
			right,
		}
	}
}

struct Grouping {
	expression:Expr,
}
impl Grouping {
	pub fn new(
		expression:Expr,
	) -> Grouping {
		Grouping {
			Expr,
		}
	}
}

struct Literal {
	value:Object,
}
impl Literal {
	pub fn new(
		value:Object,
	) -> Literal {
		Literal {
			Object,
		}
	}
}

struct Unary {
	operator:Expr,
	right:Expr,
}
impl Unary {
	pub fn new(
		operator:Expr,
		right:Expr,
	) -> Unary {
		Unary {
			Expr,
			right,
		}
	}
}
