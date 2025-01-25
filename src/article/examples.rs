pub const EXAMPLE_INPUT: &str= "arrêté préfectoral N° 5103/2016/03 du 02/03/2016 L’exploitant propose sous 2 mois à notification, des modalités de surveillance environnementale du paramètre acide sulfurique (H2SO4) hors de la plate-forme Induslacq s’ajoutant au programme de mesure de l’impact de ses installations sur l’environnement réalisé conformément à l’article 8.2.1.2 de l’ arrêté préfectoral N° 5103/2016/03 du 02/03/2016 . Après approbation de l’inspection des installations classées, il met en œuvre ce programme de surveillance de l’acide sulfurique hors de la plate-forme Induslacq sous 3 mois à notification.";

pub const EXAMPLE_OUTPUT: &str = r#"{reasoning: "The input string was parsed based on its structure:
1. The phrase 'arrêté préfectoral' at the start indicates the article type as 'Arrêté Préfectoral'.
2. The decree number '5103/2016/03' was identified after the marker 'N°'.
3. The date '02/03/2016' was extracted and reformatted to '2016-03-02' (ISO 8601 standard).
4. The remaining text was classified as the content, preserving its original structure and meaning.",
    article_type: "Arrêté Préfectoral",
    content: "L’exploitant propose sous 2 mois à notification, des modalités de surveillance environnementale du paramètre acide sulfurique (H2SO4) hors de la plate-forme Induslacq s’ajoutant au programme de mesure de l’impact de ses installations sur l’environnement réalisé conformément à l’article 8.2.1.2 de l’arrêté préfectoral N° 5103/2016/03 du 02/03/2016. Après approbation de l’inspection des installations classées, il met en œuvre ce programme de surveillance de l’acide sulfurique hors de la plate-forme Induslacq sous 3 mois à notification.",
    article_number: "5103/2016/03",
    date: "2016-03-02"
    }
"#;
