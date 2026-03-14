use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

const STREETS: &[&str] = &[
    "10th Street","11th Street","12th Street","12th Street East","13th Street","14th Street",
    "1st Avenue","1st Street","2nd Avenue","2nd Street","2nd Street East","2nd Street North",
    "2nd Street West","3rd Avenue","3rd Street","3rd Street East","3rd Street North",
    "3rd Street West","4th Avenue","4th Street","4th Street North","4th Street South",
    "4th Street West","5th Avenue","5th Street","5th Street East","5th Street North",
    "5th Street South","5th Street West","6th Avenue","6th Street","6th Street North",
    "6th Street West","7th Avenue","7th Street","8th Avenue","8th Street","8th Street South",
    "8th Street West","9th Street","9th Street West","Academy Street","Adams Avenue",
    "Adams Street","Amherst Street","Andover Court","Ann Street","Arch Street",
    "Arlington Avenue","Ashley Court","Aspen Court","Aspen Drive","Atlantic Avenue",
    "Augusta Drive","B Street","Bank Street","Bay Street","Bayberry Drive","Beech Street",
    "Beechwood Drive","Belmont Avenue","Berkshire Drive","Brandywine Drive","Briarwood Court",
    "Briarwood Drive","Bridge Street","Bridle Court","Bridle Lane","Broad Street",
    "Broad Street West","Broadway","Brook Lane","Brookside Drive","Brown Street",
    "Buckingham Drive","Buttonwood Drive","Cambridge Court","Cambridge Drive","Cambridge Road",
    "Canal Street","Canterbury Court","Canterbury Drive","Canterbury Road","Cardinal Drive",
    "Carriage Drive","Catherine Street","Cedar Avenue","Cedar Court","Cedar Lane","Cedar Street",
    "Cemetery Road","Center Street","Central Avenue","Chapel Street","Charles Street",
    "Cherry Lane","Cherry Street","Chestnut Avenue","Chestnut Street","Church Road",
    "Church Street","Church Street North","Church Street South","Circle Drive","Clark Street",
    "Clay Street","Cleveland Avenue","Cleveland Street","Clinton Street","Cobblestone Court",
    "College Avenue","College Street","Colonial Avenue","Colonial Drive","Columbia Street",
    "Cooper Street","Cottage Street","Country Club Drive","Country Club Road","Country Lane",
    "Court Street","Creek Road","Creekside Drive","Crescent Street","Cross Street",
    "Cypress Court","Deerfield Drive","Delaware Avenue","Depot Street","Devon Court",
    "Devon Road","Devonshire Drive","Division Street","Dogwood Drive","Dogwood Lane",
    "Durham Court","Durham Road","Eagle Road","Eagle Street","East Avenue","East Street",
    "Edgewood Drive","Edgewood Road","Elizabeth Street","Elm Avenue","Elm Street",
    "Elmwood Avenue","Essex Court","Euclid Avenue","Evergreen Drive","Evergreen Lane",
    "Fairview Avenue","Fairview Road","Fairway Drive","Fawn Court","Fawn Lane",
    "Fieldstone Drive","Forest Avenue","Forest Drive","Forest Street","Franklin Avenue",
    "Franklin Court","Franklin Street","Front Street","Front Street North","Front Street South",
    "Fulton Street","Garden Street","Garfield Avenue","George Street","Glenwood Avenue",
    "Glenwood Drive","Grand Avenue","Grant Avenue","Grant Street","Green Street",
    "Grove Avenue","Grove Street","Hamilton Road","Hamilton Street","Hanover Court",
    "Harrison Avenue","Harrison Street","Hartford Road","Hawthorne Avenue","Hawthorne Lane",
    "Heather Court","Heather Lane","Henry Street","Heritage Drive","Hickory Lane",
    "Hickory Street","High Street","Highland Avenue","Highland Drive","Hill Street",
    "Hillcrest Avenue","Hillcrest Drive","Hillside Avenue","Hillside Drive","Hilltop Road",
    "Holly Court","Holly Drive","Homestead Drive","Howard Street","Hudson Street",
    "Inverness Drive","Ivy Court","Ivy Lane","Jackson Avenue","Jackson Street","James Street",
    "Jefferson Avenue","Jefferson Court","Jefferson Street","John Street","Jones Street",
    "King Street","Lafayette Avenue","Lafayette Street","Lake Avenue","Lake Street",
    "Lakeview Drive","Lantern Lane","Laurel Drive","Laurel Lane","Laurel Street",
    "Lawrence Street","Lexington Court","Lexington Drive","Liberty Street","Lilac Lane",
    "Lincoln Avenue","Lincoln Street","Linda Lane","Linden Avenue","Linden Street",
    "Locust Lane","Locust Street","Madison Avenue","Madison Court","Madison Street",
    "Magnolia Avenue","Magnolia Court","Magnolia Drive","Maiden Lane","Main Street",
    "Main Street East","Main Street North","Main Street South","Main Street West",
    "Manor Drive","Maple Avenue","Maple Lane","Maple Street","Market Street",
    "Marshall Street","Meadow Lane","Meadow Street","Mechanic Street","Mill Road",
    "Mill Street","Monroe Drive","Monroe Street","Morris Street","Mulberry Court",
    "Mulberry Lane","Mulberry Street","Myrtle Avenue","Myrtle Street","New Street",
    "North Avenue","North Street","Oak Avenue","Oak Lane","Oak Street","Old York Road",
    "Olive Street","Orange Street","Orchard Avenue","Orchard Lane","Orchard Street",
    "Overlook Circle","Oxford Court","Oxford Road","Park Avenue","Park Drive","Park Place",
    "Park Street","Parker Street","Pearl Street","Penn Street","Pennsylvania Avenue",
    "Pheasant Run","Pin Oak Drive","Pine Street","Pleasant Street","Poplar Street",
    "Primrose Lane","Prospect Avenue","Prospect Street","Queen Street","Race Street",
    "Railroad Avenue","Railroad Street","Redwood Drive","Ridge Avenue","Ridge Road",
    "Ridge Street","River Road","River Street","Riverside Drive","Roberts Road",
    "Roosevelt Avenue","Rose Street","Rosewood Drive","Route 1","Route 10","Route 100",
    "Route 11","Route 17","Route 2","Route 20","Route 202","Route 27","Route 29","Route 30",
    "Route 32","Route 4","Route 41","Route 44","Route 5","Route 6","Route 64","Route 7",
    "Route 70","Route 9","School Street","Schoolhouse Lane","Shady Lane","Sheffield Drive",
    "Sherman Street","Sherwood Drive","Smith Street","Somerset Drive","South Street",
    "Spring Street","Spruce Avenue","Spruce Street","State Street","State Street East",
    "Strawberry Lane","Street Road","Summer Street","Summit Avenue","Summit Street",
    "Sunset Avenue","Sunset Drive","Surrey Lane","Sycamore Drive","Sycamore Lane",
    "Sycamore Street","Tanglewood Drive","Taylor Street","Union Street","Valley Drive",
    "Valley Road","Valley View Drive","Valley View Road","Victoria Court","Vine Street",
    "Virginia Avenue","Virginia Street","Wall Street","Walnut Avenue","Walnut Street",
    "Warren Avenue","Warren Street","Washington Avenue","Washington Street","Water Street",
    "West Avenue","West Street","Westminster Drive","White Street","William Street",
    "Williams Street","Willow Avenue","Willow Drive","Willow Lane","Willow Street",
    "Winding Way","Windsor Court","Windsor Drive","Wood Street","Woodland Avenue",
    "Woodland Drive","Woodland Road","York Road","York Street",
];

// (zip, city) pairs
const ZIP_CITY: &[(&str, &str)] = &[
    ("90001","Los Angeles"),("90003","Los Angeles"),("90004","Los Angeles"),
    ("90006","Los Angeles"),("90019","Los Angeles"),("90022","Los Angeles"),
    ("90026","Los Angeles"),("90034","Los Angeles"),("90037","Los Angeles"),
    ("90042","Los Angeles"),("90044","Los Angeles"),("90063","Los Angeles"),
    ("90066","Los Angeles"),("90221","Compton"),("90250","Hawthorne"),
    ("90255","Huntington Park"),("90262","Lynwood"),("90280","South Gate"),
    ("90631","La Habra"),("90640","Montebello"),("90660","Pico Rivera"),
    ("90703","Cerritos"),("90706","Bellflower"),("90723","Paramount"),
    ("90731","San Pedro"),("90744","Wilmington"),("90745","Carson"),
    ("90805","Long Beach"),("90813","Long Beach"),("91331","Pacoima"),
    ("91335","Reseda"),("91342","Sylmar"),("91343","North Hills"),
    ("91351","Canyon Country"),("91402","Panorama City"),("91405","Van Nuys"),
    ("91406","Van Nuys"),("91605","North Hollywood"),("91702","Azusa"),
    ("91706","Baldwin Park"),("91709","Chino Hills"),("91710","Chino"),
    ("91730","Rancho Cucamonga"),("91732","El Monte"),("91744","La Puente"),
    ("91745","Hacienda Heights"),("91761","Ontario"),("91762","Ontario"),
    ("91766","Pomona"),("91770","Rosemead"),("91801","Alhambra"),
    ("91910","Chula Vista"),("91911","Chula Vista"),("91950","National City"),
    ("91977","Spring Valley"),("92020","El Cajon"),("92021","El Cajon"),
    ("92054","Oceanside"),("92056","Oceanside"),("92069","San Marcos"),
    ("92071","Santee"),("92083","Vista"),("92105","San Diego"),
    ("92114","San Diego"),("92115","San Diego"),("92117","San Diego"),
    ("92126","San Diego"),("92154","San Diego"),("92201","Indio"),
    ("92324","Colton"),("92335","Fontana"),("92336","Fontana"),
    ("92345","Hesperia"),("92376","Rialto"),("92392","Victorville"),
    ("92404","San Bernardino"),("92503","Riverside"),("92509","Riverside"),
    ("92553","Moreno Valley"),("92627","Costa Mesa"),("92630","Lake Forest"),
    ("92646","Huntington Beach"),("92647","Huntington Beach"),
    ("92677","Laguna Niguel"),("92683","Westminster"),("92701","Santa Ana"),
    ("92703","Santa Ana"),("92704","Santa Ana"),("92707","Santa Ana"),
    ("92708","Fountain Valley"),("92780","Tustin"),("92801","Anaheim"),
    ("92804","Anaheim"),("92805","Anaheim"),("92840","Garden Grove"),
    ("92882","Corona"),("93030","Oxnard"),("93033","Oxnard"),
    ("93065","Simi Valley"),("93230","Hanford"),("93257","Porterville"),
    ("93274","Tulare"),("93306","Bakersfield"),("93307","Bakersfield"),
    ("93309","Bakersfield"),("93436","Lompoc"),("93535","Lancaster"),
    ("93550","Palmdale"),("93722","Fresno"),("93727","Fresno"),
    ("93905","Salinas"),("93906","Salinas"),("94015","Daly City"),
    ("94080","South San Francisco"),("94086","Sunnyvale"),("94087","Sunnyvale"),
    ("94109","San Francisco"),("94110","San Francisco"),("94112","San Francisco"),
    ("94122","San Francisco"),("94501","Alameda"),("94509","Antioch"),
    ("94533","Fairfield"),("94536","Fremont"),("94538","Fremont"),
    ("94541","Hayward"),("94544","Hayward"),("94550","Livermore"),
    ("94558","Napa"),("94565","Pittsburg"),("94587","Union City"),
    ("94591","Vallejo"),("94601","Oakland"),("94806","San Pablo"),
    ("95014","Cupertino"),("95035","Milpitas"),("95051","Santa Clara"),
    ("95076","Watsonville"),("95111","San Jose"),("95112","San Jose"),
    ("95116","San Jose"),("95122","San Jose"),("95123","San Jose"),
    ("95127","San Jose"),("95340","Merced"),("95350","Modesto"),
    ("95376","Tracy"),("95608","Carmichael"),("95616","Davis"),
    ("95630","Folsom"),("95687","Vacaville"),("95823","Sacramento"),
    ("95828","Sacramento"),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressConfig {
    pub street_shortcut: String,
    pub city_shortcut: String,
    pub state_shortcut: String,
    pub zip_shortcut: String,
}

impl Default for AddressConfig {
    fn default() -> Self {
        Self {
            street_shortcut: String::new(),
            city_shortcut: String::new(),
            state_shortcut: String::new(),
            zip_shortcut: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedAddress {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}

pub struct AddressState {
    pub config: Mutex<AddressConfig>,
    pub current: Mutex<GeneratedAddress>,
    pub config_path: PathBuf,
}

impl AddressState {
    pub fn new(data_dir: &PathBuf) -> Self {
        let config_path = data_dir.join("address_config.json");
        let config = load_config(&config_path);
        let current = generate_random_address();
        Self {
            config: Mutex::new(config),
            current: Mutex::new(current),
            config_path,
        }
    }

    pub fn save_config(&self) -> Result<(), String> {
        let config = self.config.lock().map_err(|e| e.to_string())?;
        let json = serde_json::to_string_pretty(&*config).map_err(|e| e.to_string())?;
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&self.config_path, json).map_err(|e| e.to_string())?;
        Ok(())
    }
}

fn load_config(path: &PathBuf) -> AddressConfig {
    if let Ok(data) = fs::read_to_string(path) {
        if let Ok(config) = serde_json::from_str::<AddressConfig>(&data) {
            return config;
        }
    }
    AddressConfig::default()
}

pub fn generate_random_address() -> GeneratedAddress {
    use std::time::SystemTime;
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as usize;

    let house_num = (seed % 9000) + 100; // 100-9099
    let street_idx = (seed / 7) % STREETS.len();
    let zip_idx = (seed / 13) % ZIP_CITY.len();

    let (zip, city) = ZIP_CITY[zip_idx];

    GeneratedAddress {
        street: format!("{} {}", house_num, STREETS[street_idx]),
        city: city.to_string(),
        state: "California".to_string(),
        zip: zip.to_string(),
    }
}

#[tauri::command]
pub fn get_address(state: tauri::State<'_, AddressState>) -> GeneratedAddress {
    state.current.lock().unwrap().clone()
}

#[tauri::command]
pub fn get_address_config(state: tauri::State<'_, AddressState>) -> AddressConfig {
    state.config.lock().unwrap().clone()
}

#[tauri::command]
pub fn new_random_address(state: tauri::State<'_, AddressState>) -> GeneratedAddress {
    let addr = generate_random_address();
    *state.current.lock().unwrap() = addr.clone();
    addr
}

#[tauri::command]
pub fn set_address_shortcut(
    app: tauri::AppHandle,
    state: tauri::State<'_, AddressState>,
    field: String,
    shortcut: String,
) -> Result<(), String> {
    {
        let mut config = state.config.lock().map_err(|e| e.to_string())?;
        match field.as_str() {
            "street" => config.street_shortcut = shortcut,
            "city" => config.city_shortcut = shortcut,
            "state" => config.state_shortcut = shortcut,
            "zip" => config.zip_shortcut = shortcut,
            _ => return Err("field must be street, city, state, or zip".into()),
        }
    }
    state.save_config()?;
    crate::hotkeys::reregister_hotkeys(&app)?;
    Ok(())
}
