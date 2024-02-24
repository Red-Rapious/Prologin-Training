(**
   @param n le nombre de dieux
   @param dieux liste des prénoms et noms des dieux séparés par un espace
   @param m nombre de passations du message
   @param passations liste des échanges de message entre les dieux, les noms complets des deux dieux séparés par un espace
*)
let cheminValide n dieux m passations =
  (** TODO Si le message n'a pas été passé en respectant le protocole, afficher
  sur une ligne le message `NON`. Sinon, afficher `OUI` sur une ligne, puis, en
  affichant un nom par ligne, le nom de tous les dieux ayant pu être dieu
  initial.  *)

  (* correspondance entre les dieux et leurs noms *)
  let noms_dieux = Hashtbl.create n in 
  List.iteri (fun i dieu -> Hashtbl.add noms_dieux dieu i) dieux ;
  
  (* initialisation du graphe *)
  let _graph = Array.make n [] in
  List.iter (
    fun str -> ()
    ) passations ;

  (* analyse *)
  let dieux_possibles = ref [] in
  List.iter (
    fun dieu_initial -> ()
  ) dieux ;

  (* fin *)
  if List.length !dieux_possibles = 0 then print_endline "NON"
  else begin 
    print_endline "OUI" ;
    List.iter (fun dieu -> print_endline dieu) !dieux_possibles
  end

let () =
  let n = read_int () in
  let dieux = List.init n (fun _ -> read_line ()) in
  let m = read_int () in
  let passations = List.init m (fun _ -> read_line ()) in
  cheminValide n dieux m passations