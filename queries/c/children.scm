;; struct
(
  struct_specifier
      name: (_) @struct.name
      body: (
          field_declaration_list (
              field_declaration
                  type: (_)
                  declarator: (_) @struct.field
          )
      )
)@struct.whole

;; TODO: enum
