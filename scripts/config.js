require('dotenv').config();

exports.STOCK_IDS = [
    // 1. market - si
    "SI0031103805", "SI0031102120", "SI0031101346", "SI0021117344",
    "SI0031102153", "SI0021110513", "SI0031104290", "SI0021111651",
    // 2. market - si
    "SI0031108994", "SI0031117813",

    // 1. market - hr
    "HRADPLRA0006", "HRARNTRA0004", "HRATGRRA0003", "HRHT00RA0005",
    "HRPODRRA0004", "HRRIVPRA0000",
    // 2. market - hr
    "HRSPANRA0007", "HRIG00RA0009", "HRLKRIRA0007", "HRMRULRA0009",
    "HRHPB0RA0002", "HRKOEIRA0009", "HRDLKVRA0006", "HRMDKARA0000",
    "HRGRNLRA0006", "HRZB00RA0003", "HRIGH0RA0006", "HRLKPCRA0005",
    // 3. market - hr
    "HRVLENRB0001", "HRERNTRA0000", "HRJDGTRA0000", "HRKRASRA0008",
    "HRAUHRRA0009", "HRKODTRA0007", "HRULPLRA0002", "HRADRSPA0009",
    "HRZABARA0009", "HRDDJHRA0007", "HRINGRRA0001", "HRPLAGRA0003",
    "HRIKBARA0008", "HRJDPLRA0007", "HRKTJVRA0002", "HRMONPRA0007",
];

exports.AT_STOCK_IDS = [
    // PRIME MARKET
    {
        type: "stock-prime-market",
        ids:[
            "addiko-bank-ag-AT000ADDIKO0", "agrana-beteiligungs-ag-AT000AGRANA3", "amag-austria-metall-ag-AT00000AMAG3",
            "andritz-ag-AT0000730007", "at-s-austria-tech-systemtech-AT0000969985", "austriacard-holdings-ag-AT0000A325L0",
            "bawag-group-ag-AT0000BAWAG2", "ca-immobilien-anlagen-ag-AT0000641352", "immofinanz-ag-AT0000A21KS2", 
            "do-co-aktiengesellschaft-AT0000818802", "erste-group-bank-ag-AT0000652011", "eurotelesites-ag-AT000000ETS9", 
            "evn-ag-AT0000741053", "facc-ag-AT00000FACC2", "flughafen-wien-ag-AT00000VIE62", 
            "frequentis-ag-ATFREQUENT09", "kapsch-trafficcom-ag-AT000KAPSCH9", "lenzing-ag-AT0000644505", 
            "mayr-melnhof-karton-ag-AT0000938204", "oesterreichische-post-ag-AT0000APOST4", "omv-ag-AT0000743059", 
            "palfinger-ag-AT0000758305", "pierer-mobility-ag-AT0000KTMI02", "polytec-holding-ag-AT0000A00XX9", 
            "porr-ag-AT0000609607", "raiffeisen-bank-internat-ag-AT0000606306", "rhi-magnesita-nv-NL0012650360", 
            "rosenbauer-international-ag-AT0000922554", "schoeller-bleckmann-ag-AT0000946652", "semperit-ag-holding-AT0000785555", 
            "strabag-se-AT000000STR1", "telekom-austria-ag-AT0000720008", "ubm-development-ag-AT0000815402", 
            "uniqa-insurance-group-ag-AT0000821103", "verbund-ag-kat-a-AT0000746409", "vienna-insurance-group-ag-AT0000908504", 
            "voestalpine-ag-AT0000937503", "wienerberger-ag-AT0000831706", "zumtobel-group-ag-AT0000837307"
        ]
    },
    /*
    // STANDARD MARKET
    {
        type: "stock-standard-market",
        ids:[
            "wolford-ag-AT0000834007", "warimpex-finanz-und-bet-ag-AT0000827209", "marinomed-biotech-ag-ATMARINOMED6"
        ]
    },

    // DIRECT MARKET PLUS
    {
        type: "stock-direct-market-plus",
        ids:[
            "steyr-motors-ag-AT0000A3FW25", "biogena-group-invest-ag-ATBIOGENA005",
        ]
    },

    // DIRECT MARKET
    {
        type: "stocks-direct-market",
        ids:[
            "enry-s-island-spa-sb-IT0005504458"
        ]
    },*/
];

exports.AT_INDEX_IDS = [
    "atx-tr-AT0000A09FJ6", "atx-AT0000999982",
    "atx-five-AT0000634605", "iatx-AT0000803226", "wbi-AT0000999990"
];

exports.INDEX_IDS = [
    "SI0026109882", "SI0028409892", "HRZB00ICBEX6", "HRZB00ICBTR6",
    "HRZB00ICBE11", "HRZB00ICB103", "HRZB00ICBPR4", "HRZB00IADPR4",
];

exports.DB_OPTIONS = {
    host: process.env.DB_HOST,
    user: process.env.DB_USER,
    password: process.env.DB_PASSWORD,
    database: process.env.DB_NAME,
};