#[derive(Debug, Clone, strum_macros::Display)]
pub enum AmazonProductCategory {
    #[strum(serialize = "本・漫画・雑誌")]
    Book(BookCategory),
    #[strum(serialize = "DVD・ミュージック・ゲーム")]
    DVDAndMusicAndGame(DVDAndMusicAndGameCategory),
    #[strum(serialize = "家電＆カメラ")]
    ElectronicsAndCamera(ElectronicsAndCameraCategory),
    #[strum(serialize = "パソコン・オフィス用品")]
    PCAndOfficeSupplies(PCAndOfficeSuppliesCategory),
    #[strum(serialize = "ホーム＆キッチン")]
    HomeAndKitchen(HomeAndKitchenCategory),
    #[strum(serialize = "食品＆飲料")]
    FoodAndDrink(FoodAndDrinkCategory),
    #[strum(serialize = "ヘルス＆ビューティー")]
    HealthAndBeauty(HealthAndBeautyCategory),
    #[strum(serialize = "ベビー・おもちゃ・ホビー")]
    BabyAndToyAndHobby(BabyAndToyAndHobbyCategory),
    #[strum(serialize = "ファッション・時計")]
    FashionAndWatch(FashionAndWatchCategory),
    #[strum(serialize = "スポーツ＆アウトドア")]
    SportsAndOutdoor(SportsAndOutdoorCategory),
    #[strum(serialize = "DIY・工具・車用品")]
    DIYAndToolsAndCar(DIYAndToolsAndCarCategory),
}

#[derive(Debug, Clone, strum_macros::Display)]
pub enum BookCategory {
    #[strum(serialize = "本")]
    Book,
    #[strum(serialize = "洋書")]
    ForeignBook,
    #[strum(serialize = "漫画")]
    Manga,
    #[strum(serialize = "雑誌")]
    Magazine,
    #[strum(serialize = "古本")]
    UsedBook,
}

#[derive(Debug, Clone, strum_macros::Display)]
pub enum DVDAndMusicAndGameCategory {
    #[strum(serialize = "DVD")]
    DVD,
    #[strum(serialize = "ブルーレイ")]
    BluRay,
    #[strum(serialize = "ミュージック")]
    Music,
    #[strum(serialize = "TVゲーム")]
    TVGame,
    #[strum(serialize = "PCゲーム")]
    PCGame,
    #[strum(serialize = "楽器")]
    Instrument,
}

#[derive(Debug, Clone, strum_macros::Display)]
pub enum ElectronicsAndCameraCategory {
    #[strum(serialize = "カメラ・デジタルカメラ")]
    CameraAndDigitalCamera,
    #[strum(serialize = "テレビ・レコーダー")]
    TVAndRecorder,
    #[strum(serialize = "オーディオ")]
    Audio,
    #[strum(serialize = "生活家電")]
    HomeAppliance,
    #[strum(serialize = "理美容家電")]
    BeautyAppliance,
    #[strum(serialize = "携帯電話")]
    MobilePhone,
    #[strum(serialize = "すべての家電＆カメラ")]
    AllElectronicsAndCamera,
}

#[derive(Debug, Clone, strum_macros::Display)]
pub enum PCAndOfficeSuppliesCategory {
    #[strum(serialize = "パソコン・周辺機器")]
    PCAndPeripheral,
    #[strum(serialize = "PCソフト")]
    PCSoftware,
    #[strum(serialize = "PCゲーム")]
    PCGame,
    #[strum(serialize = "文房具・オフィス用品")]
    StationeryAndOfficeSupplies,
}

#[derive(Debug, Clone, strum_macros::Display)]
pub enum HomeAndKitchenCategory {
    #[strum(serialize = "キッチン＆テーブルウェア")]
    KitchenAndTableware,
    #[strum(serialize = "インテリア・収納・寝具")]
    InteriorAndStorageAndBedding,
    #[strum(serialize = "生活雑貨・ペット用品")]
    HouseholdGoodsAndPetSupplies,
    #[strum(serialize = "生活家電")]
    HomeAppliance,
    #[strum(serialize = "すべてのホーム＆キッチン")]
    AllHomeAndKitchen,
}

#[derive(Debug, Clone, strum_macros::Display)]
pub enum FoodAndDrinkCategory {
    #[strum(serialize = "食品")]
    Food,
    #[strum(serialize = "スイーツ・お菓子")]
    SweetsAndSnacks,
    #[strum(serialize = "ドリンク")]
    Drink,
    #[strum(serialize = "お酒")]
    Alcohol,
    #[strum(serialize = "すべての食品＆飲料")]
    AllFoodAndDrink,
}

#[derive(Debug, Clone, strum_macros::Display)]
pub enum HealthAndBeautyCategory {
    #[strum(serialize = "コスメ")]
    Cosmetics,
    #[strum(serialize = "フェイス＆ボディケア")]
    FaceAndBodyCare,
    #[strum(serialize = "ヘルスケア")]
    Healthcare,
    #[strum(serialize = "サプリメント")]
    Supplement,
    #[strum(serialize = "フィットネス・トレーニング")]
    FitnessAndTraining,
    #[strum(serialize = "すべてのヘルス＆ビューティー")]
    AllHealthAndBeauty,
}

#[derive(Debug, Clone, strum_macros::Display)]
pub enum BabyAndToyAndHobbyCategory {
    #[strum(serialize = "ベビー＆マタニティ")]
    BabyAndMaternity,
    #[strum(serialize = "おもちゃ")]
    Toy,
    #[strum(serialize = "ホビー")]
    Hobby,
    #[strum(serialize = "楽器")]
    Instrument,
}

#[derive(Debug, Clone, strum_macros::Display)]
pub enum FashionAndWatchCategory {
    #[strum(serialize = "服＆ファッション小物")]
    ClothingAndFashionAccessories,
    #[strum(serialize = "シューズ＆バッグ")]
    ShoesAndBags,
    #[strum(serialize = "ジュエリー")]
    Jewelry,
    #[strum(serialize = "時計")]
    Watch,
    #[strum(serialize = "すべてのファッション＆時計")]
    AllFashionAndWatch,
}

#[derive(Debug, Clone, strum_macros::Display)]
pub enum SportsAndOutdoorCategory {
    #[strum(serialize = "自転車")]
    Bicycle,
    #[strum(serialize = "アウトドア")]
    Outdoor,
    #[strum(serialize = "フィットネス・トレーニング")]
    FitnessAndTraining,
    #[strum(serialize = "ゴルフ")]
    Golf,
    #[strum(serialize = "すべてのスポーツ＆アウトドア")]
    AllSportsAndOutdoor,
}

#[derive(Debug, Clone, strum_macros::Display)]
pub enum DIYAndToolsAndCarCategory {
    #[strum(serialize = "DIY・工具")]
    DIYAndTools,
    #[strum(serialize = "ガーデン")]
    Garden,
    #[strum(serialize = "カー・バイク用品")]
    CarAndBikeSupplies,
}
