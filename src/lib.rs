pub mod validadores {
    pub fn cpf(cpf: &str) -> bool {
        let cpf: Vec<u8> = cpf
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as u8)
            .collect();

        if cpf.len() != 11 || cpf.iter().all(|&d| d == cpf[0]) {
            return false;
        }

        let mut soma = 0;
        for i in 0..9 {
            soma += (cpf[i] as usize) * (10 - i);
        }
        let resto = (soma * 10) % 11;
        let primeiro_digito = if resto == 10 { 0 } else { resto as u8 };

        soma = 0;
        for i in 0..10 {
            soma += (cpf[i] as usize) * (11 - i);
        }
        let resto = (soma * 10) % 11;
        let segundo_digito = if resto == 10 { 0 } else { resto as u8 };

        primeiro_digito == cpf[9] && segundo_digito == cpf[10]
    }

    pub fn cnpj(cnpj: &str) -> bool {
        let vec: Vec<u8> = cnpj
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as u8)
            .collect();
    
        if vec.len() != 14 || vec.iter().all(|&d| d == vec[0]) {
            return false;
        }
    
        let validate_digit = |digit: usize| -> bool {
            let mut sum = 0;
            let mut weight = digit - 7;
            for &num in vec.iter().take(digit) {
                sum += num as usize * weight;
                weight = if weight == 2 { 9 } else { weight - 1 };
            }
            let remainder = sum % 11;
            let digit = if remainder < 2 { 0 } else { 11 - remainder };
            digit == vec[digit - 1] as usize
        };
    
        validate_digit(13) && validate_digit(14)
    }
}