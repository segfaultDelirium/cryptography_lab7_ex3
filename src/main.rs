use rand::Rng; // 0.8.5

fn add_modulo2(x: u8, y: u8) -> u8 {
    let res = x + y;
    if res == 2 {0} else {res}
}


fn modulo_n_vectors(x: &Vec<u8>, y: &Vec<u8>) -> Vec<u8>{
    x.iter().zip(y.into_iter()).map(|(x_el, y_el)| add_modulo2(*x_el, *y_el) ).collect()
}


fn create_hex_binary(hex_value: u8) -> Vec<u8>{
    fn create_hex_binary_rec(hex_value: u8, counter: i32, acc: Vec<u8>) -> Vec<u8>{
        if counter < 0 {
            return acc;
        }
        let two_value = (2 as u32).pow(counter as u32) as u8;
        let new_counter = counter - 1;
        if hex_value >= two_value{
            let new_acc = functional_push_right(acc, 1);
            create_hex_binary_rec(hex_value - two_value, new_counter, new_acc)
        }else{
            let new_acc = functional_push_right(acc, 0);
            create_hex_binary_rec(hex_value, new_counter, new_acc)
        }
    }
    create_hex_binary_rec(hex_value, 3, vec![])
}

fn functional_push_right(vec: Vec<u8>, value: u8) -> Vec<u8> {
    // vec.into_iter().chain([value].into_iter()).collect()
    let mut vec_clone = vec.clone();
    vec_clone.push(value);
    vec_clone
}

fn convert_from_flatten(vec: &Vec<u8>) -> Vec<Vec<u8>>{

    let to_push1 = vec.clone().into_iter().take(4).collect();
    let to_push2 = vec.clone().into_iter().skip(4).take(4).collect();
    let to_push3 = vec.clone().into_iter().skip(8).take(4).collect();
    let to_push4 = vec.clone().into_iter().skip(12).take(4).collect();
    vec![to_push1, to_push2, to_push3, to_push4]
}

fn binary_hex_to_value(binary_hex: &Vec<u8>) -> u8{
    8 * binary_hex.get(0).unwrap() + 4 * binary_hex.get(1).unwrap() + 2 * binary_hex.get(2).unwrap() + binary_hex.get(3).unwrap()
}

fn get_sbox() -> Vec<u8>{
    vec![0xE, 0x4, 0xD, 0x1, 0x2, 0xF, 0xB, 0x8, 0x3, 0xA, 0x6, 0xC, 0x5, 0x9, 0x0, 0x7]
}
fn spn(x: Vec<u8>, sbox: Vec<u8>, pbox: Vec<u8>, keys: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    // println!("Key: {:?}", keys.get(0).unwrap());
    // println!();
    // println!("x: {:?}", x.clone().into_iter().map(create_hex_binary).collect::<Vec<Vec<u8>>>());
    // println!();
    let mut w = x.clone().into_iter().map(|x| create_hex_binary(x)).flatten().collect();
    // println!("w0: {:?}", convert_from_flatten(&w));
    // println!();
    let mut v4_binary: Vec<u8> = vec![];
    for i in 0..=3 {
        let current_key: Vec<Vec<u8>> = keys.clone().into_iter().skip(i).take(4).collect();
        // println!("K{}: {:?}", i+1, current_key);
        let current_key_as_binary: Vec<u8> = current_key.clone().into_iter().flatten().collect();
        let ur_as_binary = modulo_n_vectors(&w, &current_key_as_binary);
        let ur = convert_from_flatten(&ur_as_binary);
        let ur_values: Vec<u8> = ur.clone().into_iter().map(|x| binary_hex_to_value(&x)).collect();
        // println!("u{}: {:?}", i+1, ur);
        let vr_values: Vec<u8> = vec![sbox[ur_values[0] as usize], sbox[ur_values[1] as usize], sbox[ur_values[2] as usize], sbox[ur_values[3] as usize]];
        let vr: Vec<Vec<u8>> = vr_values.into_iter().map(|x| create_hex_binary(x)).collect();
        // println!("v{}: {:?}", i+1, vr);
        let vr_binary: Vec<u8> = vr.clone().into_iter().flatten().collect();
        v4_binary = vr_binary.clone();
        let new_w_binary: Vec<u8> = pbox.clone().into_iter().map(|pbox_index| vr_binary[pbox_index as usize]).collect();
        // let new_w = convert_from_flatten(&new_w_binary);
        // println!("w{}: {:?}", i+1, new_w);
        w = new_w_binary;
        // println!();
    }
    let k5: Vec<Vec<u8>> = keys.clone().into_iter().skip(4).take(4).collect();
    let k5_as_binary: Vec<u8> = k5.clone().into_iter().flatten().collect();
    let y_binary = modulo_n_vectors(&v4_binary, &k5_as_binary);
    let y: Vec<Vec<u8>> = convert_from_flatten(&y_binary);//.into_iter().map(|x| create_hex_binary(x)).collect();
    // println!();
    // println!("k5: {:?}", k5);
    // println!();
    // println!("y: {:?}", y);
    y
}

// N = m = l = 4

fn generate_table(xprim: Vec<u8>, sbox: Vec<u8>) -> Vec<Vec<Vec<u8>>> {
    let res: Vec<Vec<Vec<u8>>> = (0..=15).into_iter().map(|x| {
        let x_binary = create_hex_binary(x);
        let xstar_as_binary = modulo_n_vectors(&x_binary, &xprim);
        let xstar = binary_hex_to_value(&xstar_as_binary);

        let y = sbox[x as usize];
        let y_binary = create_hex_binary(y);

        let ystar = sbox[xstar as usize];
        let ystar_binary = create_hex_binary(ystar);

        let yprim_binary =  modulo_n_vectors(&y_binary, &ystar_binary);
        vec![x_binary, xstar_as_binary, y_binary, ystar_binary, yprim_binary]

    }).collect();

    // vec![]
    res
}

fn get_random_hex_u8() -> u8{
    let random_int = rand::thread_rng().gen_range(0..16) as u8;
    random_int
}
fn binary_hex_string_to_value(s: String) -> u8{
    let res = s.chars().into_iter().enumerate().map(|(i, x)| {
        let value = if x == '1' {1} else {0};
        (value * (2 as u32).pow(3 - i as u32)) as u8
    }).collect::<Vec<u8>>();
    res.into_iter().sum()
    // 0
}

fn get_initial_key_binary() -> Vec<Vec<u8>> {
    let initial_key = ["0011", "1010", "1001", "0100", "1101", "0110", "0011", "1111"];
    let initial_key_as_values: Vec<_> = initial_key.into_iter().map(|x| binary_hex_string_to_value(x.to_string())).collect();
    // println!("initial_key_as_values: {:?}", initial_key_as_values);
    let initial_key_as_binary_kex: Vec<Vec<u8>> = initial_key_as_values.into_iter().map(|x| create_hex_binary(x)).collect();
    return initial_key_as_binary_kex
}



// fn create_x_y_xstar_ystar() -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>){
//     let x = vec![get_random_hex_u8(), get_random_hex_u8(), get_random_hex_u8(), get_random_hex_u8()];
//     let y = spn(x.clone(), sbox.clone(), pbox.clone(), &initial_key_as_binary_kex);
//     (x, y)
// }

fn get_pbox() -> Vec<u8> {
    vec![1,5,9,13,2,6,10,14,3,7,11,15,4,8,12,16].into_iter().map(|x| x-1).collect()

}
fn get_reverse_sbox() -> Vec<u8>{
    vec![0xE, 0x3, 0x4, 0x8, 0x1, 0xC, 0xA, 0xF, 0x7, 0xD, 0x9, 0x6, 0xB, 0x2, 0x0, 0x5]
}


fn main() {
    let initial_key_as_binary_kex: Vec<Vec<u8>> = get_initial_key_binary();
    println!("initial_key_as_binary_kex: {:?}", initial_key_as_binary_kex);
    let sbox = get_sbox();
    // println!("sbox: {:?}", sbox)
    let pbox = get_pbox();

    let xprim = vec![0,0,0,0, 1,0,1,1, 0,0,0,0, 0,0,0,0];

    let x_y_xstar_ystar: Vec<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)> = (0..200).into_iter().map(|_i| {
        let x = vec![get_random_hex_u8(), get_random_hex_u8(), get_random_hex_u8(), get_random_hex_u8()];
        let x_binary = x.clone().into_iter().map(|x| create_hex_binary(x)).flatten().collect();
        let y_binary = spn(x.clone(), sbox.clone(), pbox.clone(), &initial_key_as_binary_kex)
            .into_iter().flatten().collect();
        let x_star_binary = modulo_n_vectors(&x_binary, &xprim);
        let x_star: Vec<u8> = convert_from_flatten(&x_star_binary).into_iter()
            .map(|binary| binary_hex_to_value(&binary)).collect();
        let y_star_binary: Vec<u8> =  spn(x_star.clone(), sbox.clone(), pbox.clone(), &initial_key_as_binary_kex)
            .into_iter().flatten().collect();

        (x_binary, x_star_binary, y_binary, y_star_binary)
    }).collect();

    let mut count_table = (0..=15).into_iter().map(|_i|{
        (0..=15).into_iter().map(|_j| {
            0
        }).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();
    let reverse_sbox = get_reverse_sbox();

    let mut if_true_count = 0;
    let mut if_false_count = 0;

    x_y_xstar_ystar.into_iter().for_each(|tuple| {
        let (x, xstar, y, ystar) = tuple;
        let y_values = convert_from_flatten(&y);
        let ystar_values =  convert_from_flatten(&ystar);
        if y_values[0] != ystar_values[0] || y_values[2] != ystar_values[2] {
            return;
        }

        (0..=15).into_iter().for_each(|L1| {
            (0..=15).into_iter().for_each(|L2| {

                let L1_binary = create_hex_binary(L1);
                let y2_binary = y_values[1].clone();
                let v4_2 = modulo_n_vectors(&L1_binary, &y2_binary);

                let L2_binary = create_hex_binary(L2);
                let y4_binary = y_values[3].clone();
                let v4_4 = modulo_n_vectors(&L2_binary, &y4_binary);

                let u4_2: u8 = reverse_sbox[binary_hex_to_value(&v4_2) as usize];
                let u4_2_binary = create_hex_binary(u4_2);
                let u4_4: u8 = reverse_sbox[binary_hex_to_value(&v4_4) as usize];
                let u4_4_binary = create_hex_binary(u4_4);

                // now stars
                let y_star_2_binary = ystar_values[1].clone();
                let v4_2_star = modulo_n_vectors(&L1_binary, &y_star_2_binary);

                let y_star_4_binary = ystar_values[3].clone();
                let v4_4_star = modulo_n_vectors(&L2_binary, &y_star_4_binary);

                let u4_2_star: u8 = reverse_sbox[binary_hex_to_value(&v4_2_star) as usize];
                let u4_2_star_binary = create_hex_binary(u4_2_star);
                let u4_4_star: u8 = reverse_sbox[binary_hex_to_value(&v4_4_star) as usize];
                let u4_4_star_binary = create_hex_binary(u4_4_star);

                let u4_2_prim = modulo_n_vectors(&u4_2_binary, &u4_2_star_binary);
                let u4_4_prim = modulo_n_vectors(&u4_4_binary, &u4_4_star_binary);
                if u4_2_prim == vec![0,1,1,0] && u4_4_prim == vec![0,1,1,0]{
                    if_true_count += 1;
                    count_table[L1 as usize][L2 as usize] = count_table[L1 as usize][L2 as usize] + 1;
                }else {
                    if_false_count += 1;
                }

            })
        });

    });
    println!("if true count = {}", if_true_count);
    println!("if false count = {}", if_false_count);
    let mut max = -1;
    let mut max_key = (0, 0);
    (0..=15).into_iter().for_each(|i|{
        (0..=15).into_iter().for_each(|j| {
            if count_table[i][j] > max {
                // println!("max = {:?}", max);
                max = count_table[i][j];
                max_key = (i, j)
            };

        });
    });
    println!("max: {:?}", max);
    println!("max_key: {:?}", max_key);
    println!("key [5..=8]: {:?}", create_hex_binary(max_key.0 as u8));
    println!("key [13..=16]: {:?}", create_hex_binary(max_key.1 as u8));
    println!("bye");
}
