use egrep::regex::Regex;

#[test]
fn matcheo_con_expresion_vacia() {
    let regex = Regex::new("").unwrap();

    assert!(regex.matchea("grep"));
    assert!(regex.matchea("grep "));
    assert!(regex.matchea(" grep"));
    assert!(regex.matchea("aidofbadioh"));
}

#[test]
fn matcheo_con_literales() {
    let regex = Regex::new("grep").unwrap();

    assert!(regex.matchea("grep"));
    assert!(regex.matchea("grep "));
    assert!(regex.matchea(" grep"));
    assert!(!regex.matchea("gre"));
}

#[test]
fn matcheo_con_comodin() {
    let regex = Regex::new("gr.p").unwrap();

    assert!(regex.matchea("grep"));
    assert!(regex.matchea("grep "));
    assert!(regex.matchea(" gr.p"));
    assert!(regex.matchea("gr p"));
    assert!(regex.matchea("gr,p"));
    assert!(regex.matchea("gr3p"));
    assert!(regex.matchea("gr@p"));
    assert!(!regex.matchea("grp"));
}

#[test]
fn matcheo_desde_el_principio() {
    let regex = Regex::new("^grep").unwrap();

    assert!(regex.matchea("grep"));
    assert!(regex.matchea("grep "));
    assert!(!regex.matchea(" grep"));
}

#[test]
fn matcheo_con_final() {
    let regex = Regex::new("grep$").unwrap();

    assert!(regex.matchea("grep"));
    assert!(!regex.matchea("grep "));
    assert!(regex.matchea(" grep"));
}

#[test]
fn matcheo_con_repeticion_asterisco() {
    let regex = Regex::new("gre*p").unwrap();

    assert!(regex.matchea("greeeeeeeeeeeeep"));
    assert!(regex.matchea("grep"));
    assert!(regex.matchea("grp"));
}

#[test]
fn matcheo_con_repeticion_signo_pregunta() {
    let regex = Regex::new("gre?p").unwrap();

    assert!(!regex.matchea("greeeeeeeeeeeeep"));
    assert!(regex.matchea("grep"));
    assert!(regex.matchea("grp"));
}

#[test]
fn matcheo_con_repeticion_signo_suma() {
    let regex = Regex::new("gre+p").unwrap();

    assert!(regex.matchea("greeeeeeeeeeeeep"));
    assert!(regex.matchea("grep"));
    assert!(!regex.matchea("grp"));
}

#[test]
fn matcheo_con_repeticion_exacta() {
    let regex = Regex::new("gre{4}p").unwrap();

    assert!(!regex.matchea("greeep"));
    assert!(regex.matchea("greeeep"));
    assert!(!regex.matchea("greeeeep"));
}

#[test]
fn matcheo_con_rango_cerrado_de_repeticion() {
    let regex = Regex::new("gre{2,4}p").unwrap();

    assert!(!regex.matchea("grp"));
    assert!(!regex.matchea("grep"));
    assert!(regex.matchea("greep"));
    assert!(regex.matchea("greeep"));
    assert!(regex.matchea("greeeep"));
    assert!(!regex.matchea("greeeeep"));
}

#[test]
fn matcheo_con_rango_repeticion_abierto_a_izquierda() {
    let regex = Regex::new("gre{,4}p").unwrap();

    assert!(regex.matchea("grp"));
    assert!(regex.matchea("grep"));
    assert!(regex.matchea("greep"));
    assert!(regex.matchea("greeep"));
    assert!(regex.matchea("greeeep"));
    assert!(!regex.matchea("greeeeep"));
}

#[test]
fn matcheo_con_rango_repeticion_abierto_a_derecha() {
    let regex = Regex::new("gre{4,}p").unwrap();

    assert!(!regex.matchea("grp"));
    assert!(!regex.matchea("grep"));
    assert!(!regex.matchea("greep"));
    assert!(!regex.matchea("greeep"));
    assert!(regex.matchea("greeeep"));
    assert!(regex.matchea("greeeeep"));
    assert!(regex.matchea("greeeeeeeeeeeeeeeeep"));
}

#[test]
fn matcheo_con_rango_repeticion_abierto() {
    let regex = Regex::new("gre{,}p").unwrap();

    assert!(regex.matchea("grp"));
    assert!(regex.matchea("grep"));
    assert!(regex.matchea("greep"));
    assert!(regex.matchea("greeep"));
    assert!(regex.matchea("greeeep"));
    assert!(regex.matchea("greeeeep"));
    assert!(regex.matchea("greeeeeeeeeeeeeeeeep"));
}

#[test]
fn matcheo_con_expresion_bracket_literales() {
    let regex = Regex::new("gr[aeiou]p").unwrap();

    assert!(regex.matchea("grap"));
    assert!(regex.matchea("grep"));
    assert!(regex.matchea("grip"));
    assert!(regex.matchea("grop"));
    assert!(regex.matchea("grup"));
    assert!(!regex.matchea("grxp"));
}

#[test]
fn matcheo_con_expresion_bracket_clases_caracter() {
    let regex = Regex::new("gr[[:punct:]]p").unwrap();

    assert!(regex.matchea("gr,p"));
    assert!(!regex.matchea("grep"));
    assert!(!regex.matchea("grp"));
    assert!(regex.matchea("gr^p"));
    assert!(regex.matchea("gr.p"));
    assert!(!regex.matchea("grap"));
}

#[test]
fn matcheo_con_expresion_bracket_negada() {
    let regex = Regex::new("gr[^aeiou[:punct:]]p").unwrap();

    assert!(!regex.matchea("grap"));
    assert!(!regex.matchea("grep"));
    assert!(!regex.matchea("grip"));
    assert!(!regex.matchea("grop"));
    assert!(!regex.matchea("grup"));
    assert!(regex.matchea("grxp"));
    assert!(!regex.matchea("gr.p"));
    assert!(!regex.matchea("gr,p"));
}

#[test]

fn matcheo_con_expresiones_alternadas() {
    let regex = Regex::new("grep$|^gr.*p").unwrap();

    assert!(regex.matchea(" grep"));
    assert!(!regex.matchea(" grap"));
    assert!(!regex.matchea(" grep "));

    assert!(regex.matchea("grap"));
    assert!(regex.matchea("grep"));
    assert!(regex.matchea("grip"));
    assert!(regex.matchea("grop"));
    assert!(regex.matchea("gruqwiebfwdbp"));
    assert!(!regex.matchea(" gruqwiebfwdbp"));
}

#[test]
fn matcheo_con_comodin_y_repeticiones() {
    let regex = Regex::new(".*e{10}").unwrap();

    assert!(regex.matchea("eeeeeeeeee"));
    assert!(regex.matchea("eeeeeeeeeeeeeeeeeee"));
    assert!(regex.matchea("asdhijdhaishdeeeeeeeeeeeeeeeeeeeasdjhajdhasjd"));

    assert!(!regex.matchea("eeeeeeeee")); // 9
}
