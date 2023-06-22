use std::num::ParseIntError;

// Definición del tipo de error personalizado.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError), // Error de creación
    ParseInt(ParseIntError), // Error de análisis de enteros
}

impl ParsePosNonzeroError {
    // Función de conversión de errores de creación a ParsePosNonzeroError
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }

    // Función de conversión de errores de análisis de enteros a ParsePosNonzeroError
    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

// Función que intenta analizar un número de tipo PositiveNonzeroInteger a partir de una cadena.
fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?; // Intentar analizar el entero y convertir el error a ParsePosNonzeroError
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation) // Crear un PositiveNonzeroInteger a partir del entero y convertir cualquier error de creación a ParsePosNonzeroError
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    // Función que crea un PositiveNonzeroInteger a partir de un entero
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative), // Valor negativo, error de creación Negative
            x if x == 0 => Err(CreationError::Zero),    // Valor cero, error de creación Zero
            x => Ok(PositiveNonzeroInteger(x as u64)), // Valor positivo, crea un PositiveNonzeroInteger válido
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // Se puede cambiar la entrada para probar otros casos de error de análisis.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        // Se puede cambiar el número de prueba para verificar otros casos válidos.
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
