use ark_bn254::Fr;
use mdsecheck::mat::{product_matrix, product_vector, unique_solution};

#[test]
fn test_product_matrix() {
    assert_eq!(
        product_matrix(
            &[[Fr::from(1), Fr::from(1)], [Fr::from(1), Fr::from(0)]],
            &[] as &[&[Fr]]
        ),
        None
    );
    assert_eq!(
        product_matrix(
            &[] as &[&[Fr]],
            &[[Fr::from(1), Fr::from(0)], [Fr::from(1), Fr::from(0)]]
        ),
        None
    );
    assert_eq!(
        product_matrix(
            &[[Fr::from(1), Fr::from(1)], [Fr::from(1), Fr::from(0)]],
            &[vec![]]
        ),
        None
    );
    assert_eq!(
        product_matrix(
            &[vec![]],
            &[[Fr::from(1), Fr::from(0)], [Fr::from(1), Fr::from(0)]]
        ),
        None
    );
    assert_eq!(
        product_matrix(
            &[vec![Fr::from(2), Fr::from(3)], vec![Fr::from(4)]],
            &[[Fr::from(2), Fr::from(3)], [Fr::from(4), Fr::from(5)]]
        ),
        None
    );
    assert_eq!(
        product_matrix(
            &[[Fr::from(2), Fr::from(3)], [Fr::from(4), Fr::from(5)]],
            &[vec![Fr::from(2)], vec![Fr::from(3), Fr::from(4)]]
        ),
        None
    );
    assert_eq!(
        product_matrix(
            &[
                [Fr::from(2), Fr::from(3), Fr::from(4)],
                [Fr::from(5), Fr::from(6), Fr::from(7)]
            ],
            &[
                [Fr::from(3), Fr::from(4), Fr::from(5)],
                [Fr::from(6), Fr::from(7), Fr::from(8)]
            ]
        ),
        None
    );
    assert_eq!(
        product_matrix(
            &[
                [Fr::from(1), Fr::from(2), Fr::from(3)],
                [Fr::from(3), Fr::from(1), Fr::from(2)],
                [Fr::from(2), Fr::from(3), Fr::from(1)]
            ],
            &[
                [Fr::from(4), Fr::from(5), Fr::from(6)],
                [Fr::from(6), Fr::from(4), Fr::from(5)],
                [Fr::from(5), Fr::from(6), Fr::from(4)]
            ]
        ),
        Some(vec![
            vec![Fr::from(31), Fr::from(31), Fr::from(28)],
            vec![Fr::from(28), Fr::from(31), Fr::from(31)],
            vec![Fr::from(31), Fr::from(28), Fr::from(31)]
        ])
    );
    assert_eq!(
        product_matrix(
            &[
                [Fr::from(4), Fr::from(5), Fr::from(6)],
                [Fr::from(5), Fr::from(6), Fr::from(7)]
            ],
            &[
                [Fr::from(3), Fr::from(4)],
                [Fr::from(4), Fr::from(5)],
                [Fr::from(6), Fr::from(7)]
            ]
        ),
        Some(vec![
            vec![Fr::from(68), Fr::from(83)],
            vec![Fr::from(81), Fr::from(99)]
        ])
    );
}

#[test]
fn test_product_vector() {
    assert_eq!(
        product_vector(&[] as &[&[Fr]], &[Fr::from(1), Fr::from(1), Fr::from(0)]),
        None
    );
    assert_eq!(
        product_vector(&[vec![]], &[Fr::from(0), Fr::from(0), Fr::from(1)]),
        None
    );
    assert_eq!(
        product_vector(
            &[[Fr::from(1), Fr::from(1)], [Fr::from(1), Fr::from(0)]],
            &[]
        ),
        None
    );
    assert_eq!(
        product_vector(
            &[vec![Fr::from(2), Fr::from(3)], vec![Fr::from(4)]],
            &[Fr::from(5), Fr::from(6)]
        ),
        None
    );
    assert_eq!(
        product_vector(
            &[
                [Fr::from(2), Fr::from(3), Fr::from(4)],
                [Fr::from(5), Fr::from(6), Fr::from(7)]
            ],
            &[Fr::from(8), Fr::from(9)]
        ),
        None
    );
    assert_eq!(
        product_vector(
            &[
                [Fr::from(1), Fr::from(2), Fr::from(3)],
                [Fr::from(4), Fr::from(5), Fr::from(6)],
                [Fr::from(7), Fr::from(8), Fr::from(9)]
            ],
            &[Fr::from(3), Fr::from(5), Fr::from(7)]
        ),
        Some(vec![Fr::from(34), Fr::from(79), Fr::from(124)])
    );
    assert_eq!(
        product_vector(
            &[
                [Fr::from(1), Fr::from(2), Fr::from(1)],
                [Fr::from(3), Fr::from(1), Fr::from(4)],
                [Fr::from(1), Fr::from(5), Fr::from(1)],
                [Fr::from(6), Fr::from(1), Fr::from(7)]
            ],
            &[Fr::from(2), Fr::from(3), Fr::from(5)]
        ),
        Some(vec![Fr::from(13), Fr::from(29), Fr::from(22), Fr::from(50)])
    );
}

#[test]
fn test_unique_solution() {
    assert_eq!(
        unique_solution(&[] as &[&[Fr]], &[Fr::from(1), Fr::from(2), Fr::from(3)]),
        None
    );
    assert_eq!(
        unique_solution(&[vec![]], &[Fr::from(3), Fr::from(2), Fr::from(1)]),
        None
    );
    assert_eq!(
        unique_solution(
            &[[Fr::from(2), Fr::from(1)], [Fr::from(1), Fr::from(0)]],
            &[]
        ),
        None
    );
    assert_eq!(
        unique_solution(
            &[vec![Fr::from(3), Fr::from(4)], vec![Fr::from(5)]],
            &[Fr::from(6), Fr::from(7)]
        ),
        None
    );
    assert_eq!(
        unique_solution(
            &[
                [Fr::from(1), Fr::from(2), Fr::from(3)],
                [Fr::from(4), Fr::from(5), Fr::from(6)]
            ],
            &[Fr::from(7), Fr::from(8)]
        ),
        None
    );
    assert_eq!(
        unique_solution(
            &[
                [Fr::from(1), Fr::from(2)],
                [Fr::from(2), Fr::from(3)],
                [Fr::from(3), Fr::from(4)]
            ],
            &[Fr::from(4), Fr::from(5)]
        ),
        None
    );
    assert_eq!(
        unique_solution(
            &[
                [Fr::from(1), Fr::from(2), Fr::from(3)],
                [Fr::from(4), Fr::from(5), Fr::from(6)],
                [Fr::from(7), Fr::from(8), Fr::from(9)]
            ],
            &[Fr::from(7), Fr::from(8)]
        ),
        None
    );
    assert_eq!(
        unique_solution(
            &[
                [Fr::from(1), Fr::from(2), Fr::from(3)],
                [Fr::from(4), Fr::from(5), Fr::from(6)],
                [Fr::from(6), Fr::from(9), Fr::from(12)]
            ],
            &[Fr::from(7), Fr::from(8), Fr::from(9)]
        ),
        None
    );
    assert_eq!(
        unique_solution(
            &[
                [Fr::from(1), Fr::from(2), Fr::from(4)],
                [Fr::from(1), Fr::from(3), Fr::from(9)],
                [Fr::from(1), Fr::from(4), Fr::from(16)]
            ],
            &[Fr::from(17), Fr::from(34), Fr::from(57)]
        ),
        Some(vec![Fr::from(1), Fr::from(2), Fr::from(3)])
    );
}
