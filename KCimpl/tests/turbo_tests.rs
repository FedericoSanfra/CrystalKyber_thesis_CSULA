#[cfg(test)]
mod tests {
    use super::*; // Importa tutto dal modulo superiore, dove si trovano TurboDecoder, TurboEncoder, etc.
    use kcimpl::

    #[test]
    fn test_rsc_encoder() {
        let mut rsc = RSC::new();

        let input_vector = vec![1, 1, 0, 0, 1, 0, 1, 0, 1, 1];
        let (output_vector, _) = rsc.execute(&input_vector);

        println!("\n--test_rsc_encoder--");
        println!("input_vector = {:?}", input_vector);
        println!("output_vector = {:?}", output_vector);
        println!("state = {:?}", rsc.registers());

        // Verifica che i registri siano azzerati
        assert_eq!(rsc.registers(), vec![0; rsc.registers().len()]);
    }

    #[test]
    fn test_turbo_encoder() {
        let interleaver = vec![8, 3, 7, 6, 9, 0, 2, 5, 1, 4];
        let turbo_encoder = TurboEncoder::new(interleaver.clone());

        let input_vector = vec![1, 1, 0, 0, 1, 0, 1, 0, 1, 1];
        let output_vector = turbo_encoder.execute(&input_vector);

        let expected_vector_1 = vec![1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0];
        let expected_vector_2 = vec![1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1];

        println!("\n--test_turbo_encoder--");
        println!("output = {:?}", output_vector);

        assert_eq!(output_vector.iter().step_by(3).collect::<Vec<_>>(), expected_vector_1);
        assert_eq!(output_vector.iter().skip(2).step_by(3).collect::<Vec<_>>(), expected_vector_2);
    }

    #[test]
    fn test_siso_decoder() {
        let interleaver = vec![0; 10];
        let block_size = interleaver.len() + 2;

        let encoder = TurboEncoder::new(interleaver.clone());
        let mut channel = AWGN::new(5.0);
        let mut decoder = SISODecoder::new(block_size);

        let input_vector = vec![0, 1, 0, 1, 1, 0, 1, 0, 0, 0];
        let encoded_vector = encoder.execute(&input_vector);

        let channel_vector = channel.convert_to_symbols(encoded_vector.iter().map(|&x| x as f64).collect());
        let channel_vector = channel.execute(channel_vector);

        let demultiplexed_vector = decoder.demultiplex(&channel_vector);
        let mut decoded_vector = decoder.execute(&demultiplexed_vector);
        decoded_vector = decoded_vector.iter().map(|&b| (b > 0.0) as i32).collect();

        println!("\n--test_siso_decoder--");
        println!("input_vector = {:?}", input_vector);
        println!("encoded_vector = {:?}", encoded_vector);
        println!("decoded_vector = {:?}", decoded_vector);

        assert_eq!(encoded_vector.iter().step_by(3).collect::<Vec<_>>(), decoded_vector);
    }

    #[test]
    fn test_turbo_decoder() {
        let interleaver = vec![9, 8, 5, 6, 2, 1, 7, 0, 3, 4];
        let encoder = TurboEncoder::new(interleaver.clone());
        let mut decoder = TurboDecoder::new(interleaver.clone());

        let mut channel = AWGN::new(20.0);

        let input_vector = vec![1, 1, 0, 1, 1, 0, 1, 0, 1, 0];
        let encoded_vector = encoder.execute(&input_vector);

        let channel_vector = channel.convert_to_symbols(encoded_vector.iter().map(|&x| x as f64).collect());
        let channel_vector = channel.execute(channel_vector);

        let mut decoded_vector = decoder.execute(channel_vector);
        decoded_vector = decoded_vector.iter().map(|&b| (b > 0.0) as i32).collect();

        println!("\n--test_turbo_decoder--");
        println!("input_vector = {:?}", input_vector);
        println!("encoded_vector = {:?}", encoded_vector);
        println!("decoded_vector = {:?}", decoded_vector);

        assert_eq!(encoded_vector.iter().step_by(3).collect::<Vec<_>>(), decoded_vector);
    }
}
