
struct Binary {
	Expr:left,
	Token:operator,
	Expr:right,
}
impl Binary {
	pub fn new(
		left:left,
		Token:Token,
		Expr:Expr,
	) -> Binary {
		Binary {
			left,
			Token,
			Expr,
		}
	}
}

struct Grouping {
	Expr:expression,
}
impl Grouping {
	pub fn new(
		expression:expression,
	) -> Grouping {
		Grouping {
			expression,
		}
	}
}

struct Literal {
	Object:value,
}
impl Literal {
	pub fn new(
		value:value,
	) -> Literal {
		Literal {
			value,
		}
	}
}

struct Unary {
	Token:operator,
	Expr:right,
}
impl Unary {
	pub fn new(
		operator:operator,
		Expr:Expr,
	) -> Unary {
		Unary {
			operator,
			Expr,
		}
	}
}
