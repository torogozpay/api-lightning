pub mod response {
    #![allow(non_snake_case)]

    use utoipa::ToSchema;
    use domain::models::Invoice;
    use domain::modelsext::{InvoiceFilters,InvoiceData};

    #[derive(ToSchema)]
    pub struct InvoiceDataResponse {
        pub Ok: InvoiceData,
    }

    #[derive(ToSchema)]
    pub struct InvoiceResponse {
        pub Ok: Invoice,
    }

    #[derive(ToSchema)]
    pub struct InvoicesResponse {
        pub Ok: Vec<Invoice>,
    }

    #[derive(ToSchema)]
    pub struct InvoiceFiltersResponse {
        pub Ok: InvoiceFilters,
    }

    #[derive(ToSchema)]
    pub struct InvoicesFiltersResponse {
        pub Ok: Vec<InvoiceFilters>,
    }

    #[derive(ToSchema)]
    pub struct ErrorResponse {
        pub Err: String,
    }
		
    #[derive(ToSchema)]
    pub struct BoolResponse {
        pub Ok: bool,
    }	
	
    #[derive(ToSchema)]
    pub struct StringResponse {
        pub Ok: String,
    }
}

pub mod check {
    use shared::error_handler::CustomError;

    /// Check if a &str is a int number.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_api::utils::check;
    /// match check::validate_long("2") {
    ///     Ok(n) => assert_eq!(2, n),
    ///     Err(e) => panic!("Returned Err! => {e}"),
    /// }
    /// ```
    /// ```
    /// use lib_api::utils::check;
    ///
    /// match check::validate_long("a") {
    ///     Err(e) if e.to_string() == "Error parsing string: 'a', not a valid integer" => (),
    ///     Err(e) => panic!("Returned incorrect Err! => {e}"),
    ///     Ok(_) => panic!("Returned an Ok variant!"),
    /// }
    ///```
    pub fn validate_long(int_str: &str) -> Result<i64, CustomError> {
        int_str.parse::<i64>().map_err(|_| {
            CustomError::new(
                400,
                format!("Error parsing string: '{int_str}', not a valid integer"),
            )
        })
    }

    pub fn validate_int(int_str: &str) -> Result<i32, CustomError> {
        int_str.parse::<i32>().map_err(|_| {
            CustomError::new(
                400,
                format!("Error parsing string: '{int_str}', not a valid integer"),
            )
        })
    }

    /// Check if a &str is a float number.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_api::utils::check;
    /// match check::validate_float("1.1") {
    ///     Ok(n) => assert_eq!(1.1, n),
    ///     Err(e) => panic!("Returned Err! => {e}"),
    /// }
    /// ```
    ///
    /// ```
    /// use lib_api::utils::check;
    /// match check::validate_float("a") {
    ///     Err(e) if e.to_string() == "Error parsing string: 'a', not a valid float" => (),
    ///     Err(e) => panic!("Returned incorrect Err! => {e}"),
    ///     Ok(_) => panic!("Returned an Ok variant!"),
    /// }
    ///```
    pub fn validate_float(float_str: &str) -> Result<f64, CustomError> {
        float_str.parse::<f64>().map_err(|_| {
            CustomError::new(
                400,
                format!("Error parsing string: '{float_str}', not a valid float"),
            )
        })
    }


}

