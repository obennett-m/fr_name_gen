use std::fs::File;
use std::io::{Write, self};
use rand::Rng;

fn main() -> std::io::Result<()> {
    // Prompt user for number of names to generate
    print!("How many French names would you like to generate? ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let name_count: usize = match input.trim().parse() {
        Ok(num) => {
            if num > 0 {
                num
            } else {
                println!("Please enter a positive number. Using default: 1000");
                1_000
            }
        },
        Err(_) => {
            println!("No input. Using default of 1_000");
            1_000
        }
    };

    let first_names = vec![
        // Male names
        "Abel" , "Achille" , "Adolphe" , "Adrian" , "Aimery" , "Alain" , "Alexandre" , "Alexis",
        "Alfred" , "Alphonse" , "Ambroise" , "Andre" , "Anselme" , "Anthelme" , "Antoine",
        "Armand" , "Arnald" , "Arnaud" , "Arnot" , "Arsène" , "Artur" , "Audric" , "Auguste",
        "Augustin" , "Aurelien" , "Avent" , "Baptiste" , "Barnabé" , "Barthelemy" , "Basile",
        "Bellamy" , "Benjamin" , "Benoit" , "Bernard" , "Bertin" , "Bertrand" , "Blaise" , "Brice",
        "Bruce" , "Bruno" , "Bénédicte" , "Camille" , "Charles" , "Chretien" , "Christian",
        "Christophe" , "Claude" , "Clément" , "Conrad" , "Constant" , "Constantin" , "Corin",
        "Courtney" , "Crispin" , "Curtis" , "Cyprien" , "Cyril" , "César" , "Damien" , "Daniel",
        "David" , "Denis" , "Didier" , "Dimitri" , "Dion" , "Donatien" , "Edmond" , "Edouard",
        "Emile" , "Emilien" , "Emmanuel" , "Eric" , "Ernest" , "Etienne" , "Eugene" , "Fabien",
        "Fabrice" , "Fernand" , "Firmin" , "Florent" , "Florentin" , "Florian" , "Fortuné",
        "François" , "Frédéric" , "Félix" , "Gabriel" , "Gautier" , "Gauvain" , "Gaétan" , "Geoffroy",
        "Georges" , "Gerald" , "Gerard" , "Germain" , "Ghislain" , "Gilbert" , "Gildas" , "Gilles",
        "Giraud" , "Grégoire" , "Guillaume" , "Gustave" , "Guy" , "Henri" , "Herbert" , "Hervé",
        "Hubert" , "Hugo" , "Hugues" , "Hyacinthe" , "Ignace" , "Isaak" , "Isidore" , "Jean",
        "Jacques" , "Joseph" , "Jean-Jacques" , "Jean-Louis" , "Joseph-Marie" , "Louis" , "Joachim",
        "Joel" , "Jules" , "Julien" , "Justin" , "Jérome" , "Jérémie" , "Landry" , "Laurence",
        "Laurent" , "Lionel" , "Luc" , "Lucien" , "Léo" , "Léon" , "Marc" , "Marcel" , "Marcellin",
        "Marius" , "Marlon" , "Martin" , "Matthieu" , "Maurice" , "Maxime" , "Maximilien",
        "Michel" , "Narcisse" , "Neville" , "Nicolas" , "Noel" , "Norbert" , "Octavien" , "Olivier",
        "Parfait" , "Pascal" , "Patrice" , "Paul" , "Paulin" , "Perceval" , "Philippe" , "Pierre",
        "Quentin" , "Raoul" , "Raphael" , "Raymond" , "Remi" , "Renaud" , "René" , "Richard",
        "Robert" , "Rodolphe" , "Rodrigue" , "Roland" , "Romain" , "Samuel" , "Saul" , "Sebastien",
        "Serge" , "Severin" , "Silvain" , "Silvestre" , "Simon" , "Spiro" , "Stanislas" , "Tanguy",
        "Thibault" , "Thierry" , "Thomas" , "Théodore" , "Théophile" , "Urbain" , "Valentin",
        "Valery" , "Victor" , "Vincent" , "Yves",

        // Female names
        "Emma", "Louise", "Jade", "Alice", "Chloé", "Lina", "Léa", "Mila", "Manon", "Rose",
        "Anna", "Inès", "Camille", "Lola", "Ambre", "Léna", "Zoé", "Juliette", "Julia", "Lou",
        "Sarah", "Lucie", "Mia", "Jeanne", "Romane", "Agathe", "Nina", "Charlotte", "Inaya",
        "Margaux", "Margot", "Mathilde", "Eva", "Sofia", "Léonie", "Adèle", "Clémence", "Elena",
        "Victoria", "Anaïs", "Iris", "Elise", "Océane", "Gabrielle", "Héloïse", "Salomé", "Amélia",
        "Justine", "Célia", "Roxane", "Nora", "Eden", "Julie", "Andréa", "Myriam", "Capucine",
        "Lana", "Aya", "Charline", "Stella", "Cassandre", "Sasha", "Tessa", "Lucy", "Mélissa",
        "Eline", "Naomi", "Théa", "Lily", "Violette", "Louane", "Cléa", "Céline", "Joséphine",
        "Cécile", "Déborah", "Amélie", "Eloïse", "Audrey"
    ];

    let last_names = vec![
        "Abadie", "Achard", "Adamo", "Allard", "Alliaume", "Ambard", "Amiot", "Arnal", "Arnoux",
        "Astier", "Aubert", "Aubry", "Auvray", "Avenier", "Avril", "Babin", "Babineau", "Bailleul",
        "Ballou", "Barraud", "Barrier", "Barthélémy", "Bastien", "Baudet", "Bayol", "Beaufils",
        "Beaufoy", "Beaulieu", "Beaumont", "Bellanger", "Bellec", "Berger", "Bergeret", "Bergeron",
        "Bernard", "Berthier", "Bertrand", "Berube", "Bidault", "Blanc", "Blanchard", "Blandin",
        "Blondel", "Blondet", "Boissel", "Boisson", "Boisvert", "Bonhomme", "Bonnet", "Bonnevie",
        "Bonnin", "Bouchard", "Bouteiller", "Bouvier", "Briand", "Brideau", "Briel", "Brossard",
        "Brun", "Brunel", "Cadet", "Caillard", "Caillaud", "Capelle", "Carlier", "Carré", "Chabert",
        "Challoner", "Charbonnier", "Chastel", "Chatal", "Chenault", "Chenevier", "Chénier", "Chesnier",
        "Chevrier", "Chiasson", "Chéron", "Chrétien", "Clavet", "Clemenceau", "Cloquet", "Collard",
        "Comeaux", "Coquard", "Coudray", "Courtois", "Cousin", "Coutard", "Cédolin", "Daguerre",
        "Daladier", "d'Allemagne", "Danis", "Daudet", "Daumier", "Dautin", "David", "Dechesne", "Defraine",
        "Delage", "Delarue", "Delorme", "Denis", "Desbois", "Desnoyers", "Devere", "Devereaux", "Deville",
        "Dorat", "Doucet", "Dubois", "Duclos", "Dufour", "Duguay", "Dumont", "Dupont", "Dupré", "Dupuis",
        "Durand", "Duval", "Esnault", "Evrard", "Farjeon", "Faudel", "Fayette", "Ferrand", "Feuillette",
        "Fleuette", "Fleury", "Fortin", "Fournier", "Frassin", "Gallais", "Garbe", "Garnier", "Gaucher",
        "Gaudet", "Gauthier", "Gervais", "Girard", "Girardin", "Giraud", "Godard", "Goussand",
        "Grellier", "Grenier", "Guerrier", "Guillemot", "Guyon", "Huguelet", "Jacqueme", "Jacquemin",
        "Jaillet", "Jantot", "Josse", "Jourdain", "Jouvin", "Labarre", "LaBranche", "Labrousse",
        "Lachance", "Lachenal", "Lacroix", "Laffitte", "Lafontaine", "Lalande", "Lambert", "Lanier",
        "Larocque", "Larousse", "Latour", "Laurent", "Lavalle", "LeVan", "Leblanc", "Lecanu", "Leclerc",
        "Lefevre", "Lejeune", "Lemaitre", "Lessard", "Lesueur", "Lizot", "Loisel", "Madoré", "Maillard",
        "Mancion", "Marais", "Marchand", "Martel", "Martin", "Martineau", "Maurice", "Meserve", "Mercier",
        "Métisse", "Meunier", "Meyet", "Michaud", "Minot", "Monteil", "Moreau", "Morjuet", "Ménard",
        "Nason", "Naudin", "Naveau", "Olivier", "Orfevre", "Ouvrard", "Paillard", "Paquet", "Pasteur",
        "Pelay", "Pelletier", "Peroché", "Perrault", "Petit", "Pierpont", "Pineau", "Poincaré", "Poirier",
        "Potier", "Pourcel", "Prevost", "Prieur", "Pépin", "Quentin", "Quenu", "Rabaud", "Ragot", "Raine",
        "Raymond", "Renaud", "Renault", "Reynaud", "Ricard", "Richard", "Riviere", "Robichaud", "Rocher",
        "Rodier", "Rossi", "Rougemont", "Roussel", "Royer", "Régnier", "Saint-Simon", "Sardou", "Saunier",
        "Sergent", "Sicard", "Signoret", "Simard", "Sordeau", "Taillefer", "Tailleur", "Tavernier",
        "Terrier", "Tessier", "Theriault", "Thibaud", "Thibodeau", "Thierry", "Théry", "Thévenet",
        "Tiquet", "Tirmont", "Tison", "Tissot", "Toussaint", "Trudeau", "Turpin", "Vaillant", "Vanier",
        "Vaux", "Vavasseur", "Verdier", "Vernet", "Vernier", "Vienneau", "Vignon", "Villejoin", "Villette",
        "Vincent", "Voisin", "Véron",
    ];

    let mut rng = rand::thread_rng();
    let mut file = File::create("french_names.csv")?;

    // Write CSV header
    writeln!(file, "firstname,lastname")?;

    // Generate random name combinations
    for _ in 0..name_count {
        let first_name = first_names[rng.gen_range(0..first_names.len())];
        let last_name = last_names[rng.gen_range(0..last_names.len())];
        writeln!(file, "{},{}", first_name, last_name)?;
    }

    println!("Successfully generated {name_count} French names in 'french_names.csv'");
    Ok(())
}
