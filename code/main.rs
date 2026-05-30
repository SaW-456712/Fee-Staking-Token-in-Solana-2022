use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, signature::{read_keypair_file, Keypair, Signer}, system_instruction, transaction::Transaction};
use spl_token_2022::{ID as TOKEN_2022_PROGRAM_ID, state::Mint, extension::ExtensionType, instruction::initialize_mint2};
use spl_token_2022::extension::transfer_fee::instruction::initialize_transfer_fee_config;
use spl_token_2022::extension::interest_bearing_mint::instruction::initialize as initialize_interest_bearing_mint;
	                      
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //подкл к RPC  devnet (в будущем mainnet)
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
	// Импорт кошельков минта и админа
	let payer = read_keypair_file("admin_keypair.json").expect("No admin file");
	let mint = read_keypair_file("token_keypair.json").expect("No token file");
	println!("Admin: {}", payer.pubkey());
	println!("Mint:  {}", mint.pubkey());
	// массив расширений для Token - 2022 которые вшиваем в структуру токена
	let exten = vec![ 
		ExtensionType::TransferFeeConfig, // Вкл поддержку налога 
		ExtensionType::InterestBearingConfig, // вкл поддержку стейкинга
	];
	//Вычисляем размер аккаунта в байтах. Базовый токен занимает 165 байт 
	//Но мы добавили расширения TransferFeeConfig и InterestBearingConfig
	// и нужно узнать кол-во байт с этими расширениями
	let token_space = ExtensionType::try_calculate_account_len::<Mint>(&exten)?;
	//Узнаем Rent-exempt (скок лампортов нужно замарозить на акке чтоб токен не удалили из сети )
	let rent_lamports = client.get_minimum_balance_for_rent_exemption(token_space)?;
	println!("Rent: {}", rent_lamports as f64 / 1_000_000_000.0);
	// Настройка токена
	let decimals = 9; // Кол-во знаков после запятой
	let fee_basis_points = 200; // стартовый налог 200 базисов = 2.0% (1000 = 100%)
	let max_fee = 5_000_000_000;// макс налог с одной транзакции: 5 токенов (с учетом 9 децимелов)
	let interest_rate = 500; // Стартовый стейкинг: 500 базисов = 5% годовых

	//Сборка инструкции (пакет для транзакции)
	// Просим системы solana создать пустой аккаунт в сети по адресу минта 
	// и выделяем память (token_space) и отдаем этот аккаунт под управление Token-2022 
	let create_account_ix = system_instruction::create_account(
		&payer.pubkey(),  //Кто платит за создание 
		&mint.pubkey(),  //Адрес создоваемого аккаунта токена
		rent_lamports,  //Сколько SOL заморозить для аренды 
		token_space as u64,  //Размер аккаунта в байтах
		&TOKEN_2022_PROGRAM_ID, // Какая программа будет владеть этим аккаунтом 
	);
	// Настраиваем расширение налога внутри созданного аккаунта
	// Важно вызвать до инициализации самого минта
	let init_transfer_fee_ix = initialize_transfer_fee_config(
		&TOKEN_2022_PROGRAM_ID, // Программа будет владеть этим аккаунтом
		&mint.pubkey(), //Адрес создоваемого аккаунта токена
		Some(&payer.pubkey()), // Указываем кто имеет право менять налог (кошелек админ)
		Some(&payer.pubkey()), // Указываем кто имеет право собирать накопленный налог (инкассатор)
		fee_basis_points, // Задаем стартовый процент (2%)
		max_fee, // Задаем максмальный лимит налога 
	)?;
	//Настройка расширения стейкинга внутри аккаунта 
	// Тоже вызвать до инициализации самого минта
	let init_interset_ix = initialize_interest_bearing_mint(
		&TOKEN_2022_PROGRAM_ID, // Программа бла бла 
		&mint.pubkey(), // Адрес создоваемого аккаунта токена бла бла 
		Some(payer.pubkey()), // Указываем, кто имеет право менять процентную ставку (наш кошелек админ)
		interest_rate, //Задаем стартовую ставку в 5% 
	)?;
	// Инициализация минта и базовые параметры токена 
	let init_mint_ix = initialize_mint2(
		&TOKEN_2022_PROGRAM_ID, // Программа бла бла 
		&mint.pubkey(), // Адрес создоваемого аккаунта токена бла бла 
		&payer.pubkey(),// Mint Authority кто может печатать новые монеты
		Some(&payer.pubkey()),// Freeze Authority кто может замораживать кошельки пользователей 
		decimals, // Задаем децималы (9)
	)?;
	// Упаковка и подпись транзакции 
	// Запрашиваем свежий блокхеш (хеш последнего блокав сети) чтоб солана знала что
	// Транзакция свежая и тд ну это же блокчейн еп 
	let recent_blockhash = client.get_latest_blockhash()?;
	println!("block hash: {}", recent_blockhash);

	// Создаем атомарный пакет (транзакцию) из четырех инструкций
	// Они выполняются либо ВСЕ вместе в один миг либо ни одна из них (если где-то будет ошибка)
	let transaction = Transaction::new_signed_with_payer(
		&[create_account_ix, init_transfer_fee_ix, init_interset_ix,init_mint_ix ], //Список интрукций Минт последним а потом по порядку
		Some(&payer.pubkey()), // кто оплачивает комиссию за сеть (газ)
		&[&payer, &mint], // Массив подписей т.к нужны подписи плательщиков и ключа самого токена
		recent_blockhash, // свежий блокхеш
	);
	// Отправка в сеть
	//нужен sol на кошельке админа 
	let sign = client.send_and_confirm_transaction(&transaction)?; // подпись и отправка штамп готово монета в сети !
	println!("\tTOKEN SIGNATURED\n\t{}",sign);	

	Ok(())
}
