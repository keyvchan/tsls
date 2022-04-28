(unit) @scope

(function_declaration) @scope

(global_binding
  (identifier) @definition)
(constant_binding 
  (identifier) @definition)
(type_bindings
  (identifier) @definition)

(function_declaration
  (prototype
    (parameter_list
      (parameters
        (parameter
          (name) @definition)))))

(identifier) @reference
