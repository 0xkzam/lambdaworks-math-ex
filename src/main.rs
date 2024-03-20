use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::default_types::FrElement;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::unsigned_integer::element::U256;


fn main() {

    let secret_key = U256::from_hex_unchecked("6C616D6264617370");
    
    let s = FrElement::new(secret_key);
    let g = <BLS12381Curve as IsEllipticCurve>::generator();
    
    let pub_key_point = g.operate_with_self(s.representative());
    let pub_key_point_affine = pub_key_point.to_affine();
    
    println!("Public Key (Affine Coordinates):");
    println!("x: {}", pub_key_point_affine.x());
    println!("y: {}", pub_key_point_affine.y());
    //println!("z: {}", pub_key_point_affine.z());

}