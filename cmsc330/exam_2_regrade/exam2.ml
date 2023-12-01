type 'a tree = Leaf of 'a | Node of 'a tree * 'a * 'a tree

let even_odd_layers t =
  let rec even_odd_layers_helper (even, odd) t = match t with
    | Leaf(x) -> (x::even, odd)
    | Node(Leaf(x), value, Leaf(y)) -> (value::even, x::y::odd)
    | Node(subtree, value, Leaf(y)) -> 
        let (curr_even, curr_odd) = even_odd_layers_helper (even, odd) subtree in
        (value::curr_odd, curr_even @ [y])
    | Node(Leaf(x), value, subtree) -> 
        let (curr_even, curr_odd) = even_odd_layers_helper (even, odd) subtree in
        (value::curr_odd, x::curr_even)
    | Node(left_subtree, value, right_subtree) -> let (even_left, odd_left) = even_odd_layers_helper (even, odd) left_subtree 
        in let (final_even, final_odd) = even_odd_layers_helper (even_left, value::odd_left) right_subtree in (final_odd, final_even)

  in even_odd_layers_helper ([], []) t

(* Define a binary tree *)
let my_tree = Node(Node(Leaf(4), 2, Leaf(5)), 1, Node(Leaf(6), 3, Leaf(7)))

(* Test the even_odd_layers function *)
let (even_list, odd_list) = even_odd_layers my_tree

(* Define a function to print a list of integers *)
let rec print_int_list = function
  | [] -> ()
  | h::t -> print_int h; print_string " "; print_int_list t

(* Print the even and odd lists *)
let () =
  print_string "Even layer elements: ";
  print_int_list even_list;
  print_newline ();
  print_string "Odd layer elements: ";
  print_int_list odd_list;
  print_newline ()

