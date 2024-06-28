use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub blog: String,
    pub kochrezept: String,
    pub vorlesung: String,
}

impl Config {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("blog".to_string(), self.blog.clone());
        map.insert("kochrezept".to_string(), self.kochrezept.clone());
        map.insert("vorlesung".to_string(), self.vorlesung.clone());
        map
    }
}

pub fn get_config() -> Config {
    Config {
    blog: r#"    
      <h1 class="text-4xl font-bold">Ein Tag im Leben eines Entwicklers</h1>
        <p class="text-gray-700 text-sm">Veröffentlicht am 23. Juni 2024 von <a href="" class="text-blue-500 hover:underline">Max Mustermann</a></p>
        
        <h2 class="text-3xl font-semibold mt-6">Morgenroutine</h2>
        <p class="text-lg">Der Tag beginnt früh um <span class="font-bold">6:00 Uhr</span>. Ein schneller Kaffee und eine kurze Überprüfung der Nachrichten sind Teil meiner täglichen Routine.</p>
        <blockquote class="italic border-l-4 border-blue-500 pl-4 text-lg text-gray-700 my-4">
            "Der frühe Vogel fängt den Wurm."
        </blockquote>
        
        <h3 class="text-2xl font-medium mt-6">Arbeitstag</h3>
        <p class="text-lg">Um 8:00 Uhr beginnt der Arbeitstag. Zuerst werden die wichtigsten Aufgaben des Tages geplant:</p>
        <ul class="list-disc pl-5 mt-2 text-lg">
            <li>Code-Reviews</li>
            <li>Feature-Entwicklung</li>
            <li>Meetings mit dem Team</li>
        </ul>
        <p class="mt-4 text-lg">In einem typischen Arbeitstag steht oft das Debuggen von Code im Mittelpunkt. Hier ein Beispiel für einen einfachen Codeausschnitt:</p>
        <pre class=" rounded text-sm overflow-x-auto">
    <code class="language-rust">
    fn main() {
        println!("Blog");
    }
    </code>
        </pre>

        <h3 class="text-2xl font-medium mt-6">Mittagspause</h3>
        <p class="text-lg">Gegen Mittag gibt es eine kurze Pause. Meistens etwas Einfaches, wie ein <a href="" class="text-blue-500 hover:underline">Sandwich</a> oder ein <a href="" class="text-blue-500 hover:underline">Salat</a>.</p>
        
        <h3 class="text-2xl font-medium mt-6">Nachmittagssitzungen</h3>
        <p class="text-lg">Der Nachmittag ist oft mit Meetings gefüllt. Hier eine Übersicht der heutigen Meetings:</p>
        <table class="min-w-full divide-y divide-gray-200 mt-4">
            <thead class="bg-gray-50">
                <tr>
                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Zeit</th>
                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Besprechung</th>
                </tr>
            </thead>
            <tbody class="bg-white divide-y divide-gray-200">
                <tr>
                    <td class="px-6 py-4 whitespace-nowrap">13:00</td>
                    <td class="px-6 py-4 whitespace-nowrap">Projektstatus-Update</td>
                </tr>
                <tr>
                    <td class="px-6 py-4 whitespace-nowrap">15:00</td>
                    <td class="px-6 py-4 whitespace-nowrap">Team Brainstorming</td>
                </tr>
            </tbody>
        </table>

        <h3 class="text-2xl font-medium mt-6">Feierabend</h3>
        <p class="text-lg">Der Arbeitstag endet gegen 18:00 Uhr. Nach der Arbeit wird oft etwas Freizeit genossen, sei es ein <span class="italic">Spaziergang im Park</span> oder ein <span class="underline">Buch lesen</span>.</p>

        <h3 class="text-2xl font-medium mt-6">Fazit</h3>
        <p class="text-lg">Ein Tag im Leben eines Entwicklers kann sehr abwechslungsreich sein. Von der frühen Morgenroutine bis zum späten Abend bietet jeder Tag neue Herausforderungen und Möglichkeiten.</p>"#.to_string(),
    kochrezept: r#"
    <h1 class="text-4xl font-bold">Leckeres Spaghetti Bolognese Rezept</h1>
    <p class="text-gray-700 text-sm">Veröffentlicht am 23. Juni 2024 von <a href="" class="text-blue-500 hover:underline">Maria Koch</a></p>

    <h2 class="text-3xl font-semibold mt-6">Zutaten</h2>
    <ul class="list-disc pl-5 mt-2 text-lg">
        <li>500g Spaghetti</li>
        <li>300g Rinderhackfleisch</li>
        <li>1 Zwiebel</li>
        <li>2 Knoblauchzehen</li>
        <li>800g stückige Tomaten (aus der Dose)</li>
        <li>2 EL Tomatenmark</li>
        <li>Salz und Pfeffer</li>
        <li>Olivenöl</li>
        <li>Frischer Basilikum</li>
        <li>Parmesan</li>
    </ul>

    <h2 class="text-3xl font-semibold mt-6">Zubereitung</h2>
    <ol class="list-decimal pl-5 mt-2 text-lg">
        <li>Spaghetti in reichlich Salzwasser al dente kochen.</li>
        <li>In einer großen Pfanne das Olivenöl erhitzen und das Hackfleisch darin krümelig anbraten.</li>
        <li>Zwiebel und Knoblauch fein hacken, zum Fleisch geben und glasig dünsten.</li>
        <li>Tomatenmark hinzufügen und kurz mit anrösten.</li>
        <li>Die stückigen Tomaten in die Pfanne geben und alles gut verrühren.</li>
        <li>Mit Salz und Pfeffer abschmecken und etwa 15 Minuten köcheln lassen.</li>
        <li>Die fertigen Spaghetti mit der Bolognese-Sauce vermengen und auf Tellern anrichten.</li>
        <li>Mit frisch geriebenem Parmesan und Basilikumblättern garnieren.</li>
    </ol>

    <h2 class="text-3xl font-semibold mt-6">Tipps</h2>
    <p class="text-lg">Für eine besonders aromatische Sauce kann man die Bolognese 1-2 Stunden bei niedriger Hitze köcheln lassen.</p>
    <blockquote class="italic border-l-4 border-blue-500 pl-4 text-lg text-gray-700 my-4">
        "Die Liebe zum Kochen beginnt mit dem Verstehen der Zutaten."
    </blockquote>
    "#.to_string(),

    vorlesung: r#"
    <h1 class="text-4xl font-bold">Notizen zur Vorlesung: Kunstgeschichte 101</h1>
    <p class="text-gray-700 text-sm">Veröffentlicht am 23. Juni 2024 von <a href="" class="text-blue-500 hover:underline">Student Max Mustermann</a></p>

    <h2 class="text-3xl font-semibold mt-6">Themenübersicht</h2>
    <ul class="list-disc pl-5 mt-3">
        <li>Einführung in die Kunstgeschichte</li>
        <li>Die Renaissance: Künstler und ihre Werke</li>
        <li>Barock und Rokoko: Merkmale und Unterschiede</li>
        <li>Moderne Kunst: Impressionismus bis Abstrakter Expressionismus</li>
    </ul>

    <h2 class="text-3xl font-semibold mt-6">Einführung in die Kunstgeschichte</h2>
    <p class="mt-3">
        Kunstgeschichte ist das Studium der Entwicklung und Geschichte der Kunstwerke durch die Zeit. Sie befasst sich 
        mit der Analyse von Gemälden, Skulpturen, Architektur und anderen Kunstformen. Ziel der Kunstgeschichte ist es, 
        die kulturellen, sozialen und politischen Zusammenhänge der jeweiligen Epoche zu verstehen und die Bedeutung der 
        Kunstwerke im historischen Kontext zu erfassen.
    </p>

    <h2 class="text-3xl font-semibold mt-6">Die Renaissance</h2>
    <p class="mt-3">
        Die Renaissance war eine kulturelle Bewegung, die im 14. Jahrhundert in Italien begann und sich über ganz Europa 
        ausbreitete. Sie zeichnete sich durch eine Wiedergeburt des Interesses an der Kunst und Kultur der Antike aus. 
        Wichtige Künstler dieser Epoche sind Leonardo da Vinci, Michelangelo und Raphael.
    </p>
    <blockquote class="border-l-4 border-gray-300 pl-4 italic text-gray-600 my-3">
        "Die Renaissance war eine Zeit der Wiederentdeckung und des Experimentierens, in der Künstler wie Leonardo da Vinci 
        die Grenzen der Kreativität und Technik neu definierten."
    </blockquote>
    <figure class="my-6">
        <img src="https://upload.wikimedia.org/wikipedia/commons/e/ec/Mona_Lisa%2C_by_Leonardo_da_Vinci%2C_from_C2RMF_retouched.jpg" alt="Mona Lisa von Leonardo da Vinci" class="rounded-lg shadow-md w-1/2 mx-auto">
        <figcaption class="text-sm text-center mt-2 text-gray-600">Abb. 1: Mona Lisa von Leonardo da Vinci</figcaption>
    </figure>

    <h2 class="text-3xl font-semibold mt-6">Barock und Rokoko</h2>
    <p class="mt-3">
        Der Barock ist bekannt für seine dramatische, detaillierte und emotionale Kunst. Er erstreckte sich vom 17. bis zum 
        Anfang des 18. Jahrhunderts. Das Rokoko folgte als ein verspielter und dekorativer Stil, der vor allem in Frankreich 
        populär war.
    </p>
    <ul class="list-disc pl-5 mt-3">
        <li>Barock: Dramatik, Detailreichtum, emotionale Intensität</li>
        <li>Rokoko: Leichtigkeit, Verspieltheit, dekorative Elemente</li>
    </ul>

    <h2 class="text-3xl font-semibold mt-6">Moderne Kunst</h2>
    <p class="mt-3">
        Die Moderne Kunst umfasst zahlreiche Bewegungen, darunter den Impressionismus, den Kubismus und den Abstrakten 
        Expressionismus. Künstler wie Claude Monet, Pablo Picasso und Jackson Pollock prägten diese Epochen durch ihre 
        innovative Herangehensweise an Farbe, Form und Technik.
    </p>
    <ol class="list-decimal pl-5 mt-3">
        <li>Impressionismus: Fokus auf Licht und Farbe</li>
        <li>Kubismus: Zerlegung und Neuordnung von Formen</li>
        <li>Abstrakter Expressionismus: Ausdruck innerer Emotionen</li>
    </ol>

    <h2 class="text-3xl font-semibold mt-6">Zusammenfassung</h2>
    <p class="mt-3">
        Die Kunstgeschichte bietet einen faszinierenden Einblick in die Entwicklung der menschlichen Kreativität. Von der 
        Renaissance bis zur modernen Kunst haben Künstler immer wieder neue Wege gefunden, ihre Sicht auf die Welt 
        darzustellen. Durch das Studium der Kunstgeschichte können wir diese Entwicklungen nachvollziehen und die Bedeutung 
        der Kunstwerke im Kontext ihrer Zeit besser verstehen.
    </p>
    "#.to_string()
        }
}
