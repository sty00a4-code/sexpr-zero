use crate::Expression;

#[test]
fn basics() {
    assert_eq!(
        Expression::try_from("2398475".to_string()),
        Ok(Expression::Int(2398475))
    );
    assert_eq!(
        Expression::try_from("78435.67768".to_string()),
        Ok(Expression::Float(78435.67768))
    );
    assert_eq!(
        Expression::try_from("name".to_string()),
        Ok(Expression::Symbol("name".to_string()))
    );
    assert_eq!(
        Expression::try_from("last-name".to_string()),
        Ok(Expression::Symbol("last-name".to_string()))
    );
    assert_eq!(
        Expression::try_from("+".to_string()),
        Ok(Expression::Symbol("+".to_string()))
    );
    assert_eq!(
        Expression::try_from("`".to_string()),
        Ok(Expression::Symbol("`".to_string()))
    );
    assert_eq!(
        Expression::try_from("\"hello there stranger\"".to_string()),
        Ok(Expression::String("hello there stranger".to_string()))
    );
    assert_eq!(
        Expression::try_from("(+ 1 2.5)".to_string()),
        Ok(Expression::List(vec![
            Expression::Symbol("+".to_string()),
            Expression::Int(1),
            Expression::Float(2.5)
        ]))
    );
    assert_eq!(
        Expression::try_from("(+ 1 (- 2 3))".to_string()),
        Ok(Expression::List(vec![
            Expression::Symbol("+".to_string()),
            Expression::Int(1),
            Expression::List(vec![
                Expression::Symbol("-".to_string()),
                Expression::Int(2),
                Expression::Int(3),
            ]),
        ]))
    );
}
