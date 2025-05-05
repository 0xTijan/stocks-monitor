use diesel::MysqlConnection;

pub struct DbService {
    pub connection: MysqlConnection,
}

impl DbService {
    pub fn new() -> Self {
        dotenv().ok();

        let sb_rul = env::var("DATABASE_URL").expect("SB_RUL must be set");

        let connection = MysqlConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

        DbService { connection }
    }

    pub fn create_stock(&mut self, stock: NewStock) {
        diesel::insert_into(schema::stocks::table)
            .values(&stock)
            .execute(&mut self.connection)
            .expect("Error saving new stock");
    }

    pub fn list_stocks(&mut self) -> Vec<Stock> {
        schema::stocks::table
            .load::<Stock>(&mut self.connection)
            .expect("Error loading stocks")
    }
}