use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Timezones
{
    region: String,
    city: Option<Vec<String>>
}

impl Timezones
{
    fn new(region: &str, city: Option<Vec<&str>>) -> Self
    {
        let region = region.to_string();

        let city: Option<Vec<String>> = match city
        {
            Some(s) => {

                let s = s
                    .iter()
                    .map(|s| s.to_string())
                    .collect();

                Some(s)
            }

            _ => None
        };

        Self { region, city }
    }

    pub fn list_json() -> String
    {
        let list = Self::list();

        let json = serde_json::to_string_pretty(&list);

        match json
        {
            Ok(s) => s,
            Err(e) => e.to_string()
        }
    }

    pub fn list() -> Vec<Self>
    {
        vec![
            Self::africa(),
            Self::america(),
            Self::antarctica(),
            Self::arctic(),
            Self::asia(),
            Self::atlantic(),
            Self::australia(),
            Self::brazil(),
            Self::canada(),
            Self::chile(),
            Self::etc(),
            Self::europe(),
            Self::indian(),
            Self::mexico(),
            Self::pacific(),
            Self::us(),
            Self::new("CET", None),
            Self::new("CST6CDT", None),
            Self::new("Cuba", None),
            Self::new("EET", None),
            Self::new("Egypt", None),
            Self::new("Eire", None),
            Self::new("EST", None),
            Self::new("EST5EDT", None),
            Self::new("Factory", None),
            Self::new("GB", None),
            Self::new("GB-Eire", None),
            Self::new("GMT", None),
            Self::new("GMT+0", None),
            Self::new("GMT-0", None),
            Self::new("GMT0", None),
            Self::new("Greenwich", None),
            Self::new("Hongkong", None),
            Self::new("HST", None),
            Self::new("Iceland", None),
            Self::new("Iran", None),
            Self::new("Israel", None),
            Self::new("Jamaica", None),
            Self::new("Japan", None),
            Self::new("Kwajalein", None),
            Self::new("Libya", None),
            Self::new("MET", None),
            Self::new("MST", None),
            Self::new("MST7MDT", None),
            Self::new("Navajo", None),
            Self::new("NZ", None),
            Self::new("NZ-CHAT", None),
            Self::new("Poland", None),
            Self::new("Portugal", None),
            Self::new("PRC", None),
            Self::new("PST8PDT", None),
            Self::new("ROC", None),
            Self::new("ROK", None),
            Self::new("Singapore", None),
            Self::new("Turkey", None),
            Self::new("UCT", None),
            Self::new("Universal", None),
            Self::new("UTC", None),
            Self::new("WET", None),
            Self::new("W-SU", None),
            Self::new("Zulu", None)
        ]
    }

    fn africa() -> Self
    {
        let city = vec![

            "Abidjan",
            "Accra",
            "Addis_Ababa",
            "Algiers",
            "Asmara",
            "Asmera",
            "Bamako",
            "Bangui",
            "Banjul",
            "Bissau",
            "Blantyre",
            "Brazzaville",
            "Bujumbura",
            "Cairo",
            "Casablanca",
            "Ceuta",
            "Conakry",
            "Dakar",
            "Dar_es_Salaam",
            "Djibouti",
            "Douala",
            "El_Aaiun",
            "Freetown",
            "Gaborone",
            "Harare",
            "Johannesburg",
            "Juba",
            "Kampala",
            "Khartoum",
            "Kigali",
            "Kinshasa",
            "Lagos",
            "Libreville",
            "Lome",
            "Luanda",
            "Lubumbashi",
            "Lusaka",
            "Malabo",
            "Maputo",
            "Maseru",
            "Mbabane",
            "Mogadishu",
            "Monrovia",
            "Nairobi",
            "Ndjamena",
            "Niamey",
            "Nouakchott",
            "Ouagadougou",
            "Porto-Novo",
            "Sao_Tome",
            "Timbuktu",
            "Tripoli",
            "Tunis",
            "Windhoek"
        ];

        Self::new("Africa", Some(city))
    }

    fn america() -> Self
    {
        let city = vec![

            "Adak",
            "Anchorage",
            "Anguilla",
            "Antigua",
            "Araguaina",
            "Argentina",
            "Aruba",
            "Asuncion",
            "Atikokan",
            "Atka",
            "Bahia",
            "Bahia_Banderas",
            "Barbados",
            "Belem",
            "Belize",
            "Blanc-Sablon",
            "Boa_Vista",
            "Bogota",
            "Boise",
            "Buenos_Aires",
            "Cambridge_Bay",
            "Campo_Grande",
            "Cancun",
            "Caracas",
            "Catamarca",
            "Cayenne",
            "Cayman",
            "Chicago",
            "Chihuahua",
            "Ciudad_Juarez",
            "Coral_Harbour",
            "Cordoba",
            "Costa_Rica",
            "Creston",
            "Cuiaba",
            "Curacao",
            "Danmarkshavn",
            "Dawson",
            "Dawson_Creek",
            "Denver",
            "Detroit",
            "Dominica",
            "Edmonton",
            "Eirunepe",
            "El_Salvador",
            "Ensenada",
            "Fortaleza",
            "Fort_Nelson",
            "Fort_Wayne",
            "Glace_Bay",
            "Godthab",
            "Goose_Bay",
            "Grand_Turk",
            "Grenada",
            "Guadeloupe",
            "Guatemala",
            "Guayaquil",
            "Guyana",
            "Halifax",
            "Havana",
            "Hermosillo",
            "Indiana",
            "Indianapolis",
            "Inuvik",
            "Iqaluit",
            "Jamaica",
            "Jujuy",
            "Juneau",
            "Kentucky",
            "Knox_IN",
            "Kralendijk",
            "La_Paz",
            "Lima",
            "Los_Angeles",
            "Louisville",
            "Lower_Princes",
            "Maceio",
            "Managua",
            "Manaus",
            "Marigot",
            "Martinique",
            "Matamoros",
            "Mazatlan",
            "Mendoza",
            "Menominee",
            "Merida",
            "Metlakatla",
            "Mexico_City",
            "Miquelon",
            "Moncton",
            "Monterrey",
            "Montevideo",
            "Montreal",
            "Montserrat",
            "Nassau",
            "New_York",
            "Nipigon",
            "Nome",
            "Noronha",
            "North_Dakota",
            "Nuuk",
            "Ojinaga",
            "Panama",
            "Pangnirtung",
            "Paramaribo",
            "Phoenix",
            "Port-au-Prince",
            "Porto_Acre",
            "Port_of_Spain",
            "Porto_Velho",
            "Puerto_Rico",
            "Punta_Arenas",
            "Rainy_River",
            "Rankin_Inlet",
            "Recife",
            "Regina",
            "Resolute",
            "Rio_Branco",
            "Rosario",
            "Santa_Isabel",
            "Santarem",
            "Santiago",
            "Santo_Domingo",
            "Sao_Paulo",
            "Scoresbysund",
            "Shiprock",
            "Sitka",
            "St_Barthelemy",
            "St_Johns",
            "St_Kitts",
            "St_Lucia",
            "St_Thomas",
            "St_Vincent",
            "Swift_Current",
            "Tegucigalpa",
            "Thule",
            "Thunder_Bay",
            "Tijuana",
            "Toronto",
            "Tortola",
            "Vancouver",
            "Virgin",
            "Whitehorse",
            "Winnipeg",
            "Yakutat",
            "Yellowknife",
        ];

        Self::new("America", Some(city))
    }

    fn antarctica() -> Self
    {
        let city = vec![

            "Casey",
            "Davis",
            "DumontDUrville",
            "Macquarie",
            "Mawson",
            "McMurdo",
            "Palmer",
            "Rothera",
            "South_Pole",
            "Syowa",
            "Troll",
            "Vostok"
        ];

        Self::new("Antarctica", Some(city))
    }

    fn arctic() -> Self
    {
        let city = vec![
            "Longyearbyen"
        ];

        Self::new("Arctic", Some(city))
    }

    fn asia() -> Self
    {
        let city = vec![

            "Aden",
            "Almaty",
            "Amman",
            "Anadyr",
            "Aqtau",
            "Aqtobe",
            "Ashgabat",
            "Ashkhabad",
            "Atyrau",
            "Baghdad",
            "Bahrain",
            "Baku",
            "Bangkok",
            "Barnaul",
            "Beirut",
            "Bishkek",
            "Brunei",
            "Calcutta",
            "Chita",
            "Choibalsan",
            "Chongqing",
            "Chungking",
            "Colombo",
            "Dacca",
            "Damascus",
            "Dhaka",
            "Dili",
            "Dubai",
            "Dushanbe",
            "Famagusta",
            "Gaza",
            "Harbin",
            "Hebron",
            "Ho_Chi_Minh",
            "Hong_Kong",
            "Hovd",
            "Irkutsk",
            "Istanbul",
            "Jakarta",
            "Jayapura",
            "Jerusalem",
            "Kabul",
            "Kamchatka",
            "Karachi",
            "Kashgar",
            "Kathmandu",
            "Katmandu",
            "Khandyga",
            "Kolkata",
            "Krasnoyarsk",
            "Kuala_Lumpur",
            "Kuching",
            "Kuwait",
            "Macao",
            "Macau",
            "Magadan",
            "Makassar",
            "Manila",
            "Muscat",
            "Nicosia",
            "Novokuznetsk",
            "Novosibirsk",
            "Omsk",
            "Oral",
            "Phnom_Penh",
            "Pontianak",
            "Pyongyang",
            "Qatar",
            "Qostanay",
            "Qyzylorda",
            "Rangoon",
            "Riyadh",
            "Saigon",
            "Sakhalin",
            "Samarkand",
            "Seoul",
            "Shanghai",
            "Singapore",
            "Srednekolymsk",
            "Taipei",
            "Tashkent",
            "Tbilisi",
            "Tehran",
            "Tel_Aviv",
            "Thimbu",
            "Thimphu",
            "Tokyo",
            "Tomsk",
            "Ujung_Pandang",
            "Ulaanbaatar",
            "Ulan_Bator",
            "Urumqi",
            "Ust-Nera",
            "Vientiane",
            "Vladivostok",
            "Yakutsk",
            "Yangon",
            "Yekaterinburg",
            "Yerevan",
        ];

        Self::new("Asia", Some(city))
    }

    fn atlantic() -> Self
    {
        let city = vec![

            "Azores",
            "Bermuda",
            "Canary",
            "Cape_Verde",
            "Faeroe",
            "Faroe",
            "Jan_Mayen",
            "Madeira",
            "Reykjavik",
            "South_Georgia",
            "Stanley",
            "St_Helena"
        ];

        Self::new("Atlantic", Some(city))
    }

    fn australia() -> Self
    {
        let city = vec![

            "ACT",
            "Adelaide",
            "Brisbane",
            "Broken_Hill",
            "Canberra",
            "Currie",
            "Darwin",
            "Eucla",
            "Hobart",
            "LHI",
            "Lindeman",
            "Lord_Howe",
            "Melbourne",
            "North",
            "NSW",
            "Perth",
            "Queensland",
            "South",
            "Sydney",
            "Tasmania",
            "Victoria",
            "West",
            "Yancowinna"
        ];

        Self::new("Australia", Some(city))
    }

    fn brazil() -> Self
    {
        let city = vec![

            "Acre",
            "DeNoronha",
            "East",
            "West"
        ];

        Self::new("Brazil", Some(city))
    }

    fn canada() -> Self
    {
        let city = vec![
            "Atlantic",
            "Central",
            "Eastern",
            "Mountain",
            "Newfoundland",
            "Pacific",
            "Saskatchewan",
            "Yukon"
        ];

        Self::new("Canada", Some(city))
    }

    fn chile() -> Self
    {
        let city = vec![

            "Continental",
            "EasterIsland"
        ];

        Self::new("Chile", Some(city))
    }

    fn europe() -> Self
    {
        let city = vec![

            "Amsterdam",
            "Andorra",
            "Astrakhan",
            "Athens",
            "Belfast",
            "Belgrade",
            "Berlin",
            "Bratislava",
            "Brussels",
            "Bucharest",
            "Budapest",
            "Busingen",
            "Chisinau",
            "Copenhagen",
            "Dublin",
            "Gibraltar",
            "Guernsey",
            "Helsinki",
            "Isle_of_Man",
            "Istanbul",
            "Jersey",
            "Kaliningrad",
            "Kiev",
            "Kirov",
            "Kyiv",
            "Lisbon",
            "Ljubljana",
            "London",
            "Luxembourg",
            "Madrid",
            "Malta",
            "Mariehamn",
            "Minsk",
            "Monaco",
            "Moscow",
            "Nicosia",
            "Oslo",
            "Paris",
            "Podgorica",
            "Prague",
            "Riga",
            "Rome",
            "Samara",
            "San_Marino",
            "Sarajevo",
            "Saratov",
            "Simferopol",
            "Skopje",
            "Sofia",
            "Stockholm",
            "Tallinn",
            "Tirane",
            "Tiraspol",
            "Ulyanovsk",
            "Uzhgorod",
            "Vaduz",
            "Vatican",
            "Vienna",
            "Vilnius",
            "Volgograd",
            "Warsaw",
            "Zagreb",
            "Zaporozhye",
            "Zurich"
        ];

        Self::new("Europe", Some(city))
    }

    fn etc() -> Self
    {
        let city = vec![
            "GMT",
            "GMT+0",
            "GMT-0",
            "GMT0",
            "GMT+1",
            "GMT-1",
            "GMT+10",
            "GMT-10",
            "GMT+11",
            "GMT-11",
            "GMT+12",
            "GMT-12",
            "GMT-13",
            "GMT-14",
            "GMT+2",
            "GMT-2",
            "GMT+3",
            "GMT-3",
            "GMT+4",
            "GMT-4",
            "GMT+5",
            "GMT-5",
            "GMT+6",
            "GMT-6",
            "GMT+7",
            "GMT-7",
            "GMT+8",
            "GMT-8",
            "GMT+9",
            "GMT-9",
            "Greenwich",
            "UCT",
            "Universal",
            "UTC",
            "Zulu"
        ];

        Self::new("Etc", Some(city))
    }

    fn indian() -> Self
    {
        let city = vec![
            "Antananarivo",
            "Chagos",
            "Christmas",
            "Cocos",
            "Comoro",
            "Kerguelen",
            "Mahe",
            "Maldives",
            "Mauritius",
            "Mayotte",
            "Reunion"
        ];

        Self::new("Indian", Some(city))
    }

    fn mexico() -> Self
    {
        let city = vec![

            "BajaNorte",
            "BajaSur",
            "General"
        ];

        Self::new("Mexico", Some(city))
    }

    fn pacific() -> Self
    {
        let city = vec![

            "Apia",
            "Auckland",
            "Bougainville",
            "Chatham",
            "Chuuk",
            "Easter",
            "Efate",
            "Enderbury",
            "Fakaofo",
            "Fiji",
            "Funafuti",
            "Galapagos",
            "Gambier",
            "Guadalcanal",
            "Guam",
            "Honolulu",
            "Johnston",
            "Kanton",
            "Kiritimati",
            "Kosrae",
            "Kwajalein",
            "Majuro",
            "Marquesas",
            "Midway",
            "Nauru",
            "Niue",
            "Norfolk",
            "Noumea",
            "Pago_Pago",
            "Palau",
            "Pitcairn",
            "Pohnpei",
            "Ponape",
            "Port_Moresby",
            "Rarotonga",
            "Saipan",
            "Samoa",
            "Tahiti",
            "Tarawa",
            "Tongatapu",
            "Truk",
            "Wake",
            "Wallis",
            "Yap"
        ];

        Self::new("Pacific", Some(city))
    }

    fn us() -> Self
    {
        let city = vec![

            "Alaska",
            "Aleutian",
            "Arizona",
            "Central",
            "Eastern",
            "East-Indiana",
            "Hawaii",
            "Indiana-Starke",
            "Michigan",
            "Mountain",
            "Pacific",
            "Samoa"
        ];

        Self::new("US", Some(city))
    }
}
