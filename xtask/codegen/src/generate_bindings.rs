use rome_js_factory::make;
use rome_js_formatter::{context::JsFormatOptions, format_node};
use rome_js_syntax::{
    JsAnyBinding, JsAnyBindingPattern, JsAnyCallArgument, JsAnyDeclaration, JsAnyDeclarationClause,
    JsAnyExportClause, JsAnyExpression, JsAnyFormalParameter, JsAnyImportClause,
    JsAnyLiteralExpression, JsAnyModuleItem, JsAnyName, JsAnyNamedImport,
    JsAnyNamedImportSpecifier, JsAnyObjectMember, JsAnyObjectMemberName, JsAnyParameter,
    JsAnyStatement, TriviaPieceKind, TsAnyName, TsAnyReturnType, TsAnyTypeMember, TsType, T,
};
use rome_rowan::AstNode;
use rome_service::workspace_types::{generate_type, methods, ModuleQueue};
use xtask::{project_root, Mode, Result};
use xtask_codegen::{to_camel_case, update};

pub(crate) fn generate_workspace_bindings(mode: Mode) -> Result<()> {
    let bindings_path = project_root().join("npm/backend-jsonrpc/src/workspace.ts");
    let methods = methods();

    let mut declarations = Vec::new();
    let mut member_definitions = Vec::with_capacity(methods.len());
    let mut member_declarations = Vec::with_capacity(methods.len());
    let mut queue = ModuleQueue::default();

    for method in &methods {
        let params = generate_type(&mut declarations, &mut queue, &method.params);
        let result = generate_type(&mut declarations, &mut queue, &method.result);

        let camel_case = to_camel_case(method.name);

        member_definitions.push(TsAnyTypeMember::TsMethodSignatureTypeMember(
            make::ts_method_signature_type_member(
                JsAnyObjectMemberName::JsLiteralMemberName(make::js_literal_member_name(
                    make::ident(&camel_case),
                )),
                make::js_parameters(
                    make::token(T!['(']),
                    make::js_parameter_list(
                        Some(JsAnyParameter::JsAnyFormalParameter(
                            JsAnyFormalParameter::JsFormalParameter(
                                make::js_formal_parameter(JsAnyBindingPattern::JsAnyBinding(
                                    JsAnyBinding::JsIdentifierBinding(make::js_identifier_binding(
                                        make::ident("params"),
                                    )),
                                ))
                                .with_type_annotation(make::ts_type_annotation(
                                    make::token(T![:]),
                                    params,
                                ))
                                .build(),
                            ),
                        )),
                        None,
                    ),
                    make::token(T![')']),
                ),
            )
            .with_return_type_annotation(make::ts_return_type_annotation(
                make::token(T![:]),
                TsAnyReturnType::TsType(TsType::TsReferenceType(
                    make::ts_reference_type(TsAnyName::JsReferenceIdentifier(
                        make::js_reference_identifier(make::ident("Promise")),
                    ))
                    .with_type_arguments(make::ts_type_arguments(
                        make::token(T![<]),
                        make::ts_type_argument_list(Some(result), None),
                        make::token(T![>]),
                    ))
                    .build(),
                )),
            ))
            .build(),
        ));

        member_declarations.push(JsAnyObjectMember::JsMethodObjectMember(
            make::js_method_object_member(
                JsAnyObjectMemberName::JsLiteralMemberName(make::js_literal_member_name(
                    make::ident(&camel_case),
                )),
                make::js_parameters(
                    make::token(T!['(']),
                    make::js_parameter_list(
                        Some(JsAnyParameter::JsAnyFormalParameter(
                            JsAnyFormalParameter::JsFormalParameter(
                                make::js_formal_parameter(JsAnyBindingPattern::JsAnyBinding(
                                    JsAnyBinding::JsIdentifierBinding(make::js_identifier_binding(
                                        make::ident("params"),
                                    )),
                                ))
                                .build(),
                            ),
                        )),
                        None,
                    ),
                    make::token(T![')']),
                ),
                make::js_function_body(
                    make::token(T!['{']),
                    make::js_directive_list(None),
                    make::js_statement_list(Some(JsAnyStatement::JsReturnStatement(
                        make::js_return_statement(make::token(T![return]))
                            .with_argument(JsAnyExpression::JsCallExpression(
                                make::js_call_expression(
                                    JsAnyExpression::JsStaticMemberExpression(
                                        make::js_static_member_expression(
                                            JsAnyExpression::JsIdentifierExpression(
                                                make::js_identifier_expression(
                                                    make::js_reference_identifier(make::ident(
                                                        "transport",
                                                    )),
                                                ),
                                            ),
                                            make::token(T![.]),
                                            JsAnyName::JsName(make::js_name(make::ident(
                                                "request",
                                            ))),
                                        ),
                                    ),
                                    make::js_call_arguments(
                                        make::token(T!['(']),
                                        make::js_call_argument_list(
                                            [
                                                JsAnyCallArgument::JsAnyExpression(
                                                    JsAnyExpression::JsAnyLiteralExpression(
                                                        JsAnyLiteralExpression::JsStringLiteralExpression(make::js_string_literal_expression(make::js_string_literal(&format!("rome/{}", method.name)))),
                                                    ),
                                                ),
                                                JsAnyCallArgument::JsAnyExpression(
                                                    JsAnyExpression::JsIdentifierExpression(
                                                        make::js_identifier_expression(
                                                            make::js_reference_identifier(make::ident(
                                                                "params",
                                                            )),
                                                        ),
                                                    ),
                                                ),
                                            ],
                                            Some(make::token(T![,])),
                                        ),
                                        make::token(T![')']),
                                    ),
                                )
                                .build(),
                            ))
                            .build(),
                    ))),
                    make::token(T!['}']),
                ),
            )
            .build(),
        ));
    }

    let leading_comment = [
        (
            TriviaPieceKind::SingleLineComment,
            "// Generated file, do not edit by hand, see `xtask/codegen`",
        ),
        (TriviaPieceKind::Newline, "\n"),
    ];

    let mut items = vec![JsAnyModuleItem::JsImport(
        make::js_import(
            make::token(T![import]).with_leading_trivia(leading_comment.into_iter()),
            JsAnyImportClause::JsImportNamedClause(
                make::js_import_named_clause(
                    JsAnyNamedImport::JsNamedImportSpecifiers(make::js_named_import_specifiers(
                        make::token(T!['{']),
                        make::js_named_import_specifier_list(
                            Some(JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(
                                make::js_shorthand_named_import_specifier(
                                    JsAnyBinding::JsIdentifierBinding(make::js_identifier_binding(
                                        make::ident("Transport"),
                                    )),
                                )
                                .build(),
                            )),
                            None,
                        ),
                        make::token(T!['}']),
                    )),
                    make::token(T![from]),
                    make::js_module_source(make::js_string_literal("./transport")),
                )
                .with_type_token(make::token(T![type]))
                .build(),
            ),
        )
        .build(),
    )];

    items.extend(declarations.into_iter().map(|(decl, description)| {
        let mut export = make::token(T![export]);
        if let Some(description) = description {
            let comment = format!("/**\n\t* {} \n\t */\n", description);
            let trivia = vec![
                (TriviaPieceKind::MultiLineComment, comment.as_str()),
                (TriviaPieceKind::Newline, "\n"),
            ];
            export = export.with_leading_trivia(trivia);
        }
        JsAnyModuleItem::JsExport(make::js_export(
            export,
            JsAnyExportClause::JsAnyDeclarationClause(match decl {
                JsAnyDeclaration::JsClassDeclaration(decl) => {
                    JsAnyDeclarationClause::JsClassDeclaration(decl)
                }
                JsAnyDeclaration::JsFunctionDeclaration(decl) => {
                    JsAnyDeclarationClause::JsFunctionDeclaration(decl)
                }
                JsAnyDeclaration::JsVariableDeclaration(decl) => {
                    JsAnyDeclarationClause::JsVariableDeclarationClause(
                        make::js_variable_declaration_clause(decl).build(),
                    )
                }
                JsAnyDeclaration::TsDeclareFunctionDeclaration(decl) => {
                    JsAnyDeclarationClause::TsDeclareFunctionDeclaration(decl)
                }
                JsAnyDeclaration::TsEnumDeclaration(decl) => {
                    JsAnyDeclarationClause::TsEnumDeclaration(decl)
                }
                JsAnyDeclaration::TsExternalModuleDeclaration(decl) => {
                    JsAnyDeclarationClause::TsExternalModuleDeclaration(decl)
                }
                JsAnyDeclaration::TsGlobalDeclaration(decl) => {
                    JsAnyDeclarationClause::TsGlobalDeclaration(decl)
                }
                JsAnyDeclaration::TsImportEqualsDeclaration(decl) => {
                    JsAnyDeclarationClause::TsImportEqualsDeclaration(decl)
                }
                JsAnyDeclaration::TsInterfaceDeclaration(decl) => {
                    JsAnyDeclarationClause::TsInterfaceDeclaration(decl)
                }
                JsAnyDeclaration::TsModuleDeclaration(decl) => {
                    JsAnyDeclarationClause::TsModuleDeclaration(decl)
                }
                JsAnyDeclaration::TsTypeAliasDeclaration(decl) => {
                    JsAnyDeclarationClause::TsTypeAliasDeclaration(decl)
                }
            }),
        ))
    }));

    member_definitions.push(TsAnyTypeMember::TsMethodSignatureTypeMember(
        make::ts_method_signature_type_member(
            JsAnyObjectMemberName::JsLiteralMemberName(make::js_literal_member_name(make::ident(
                "destroy",
            ))),
            make::js_parameters(
                make::token(T!['(']),
                make::js_parameter_list(None, None),
                make::token(T![')']),
            ),
        )
        .with_return_type_annotation(make::ts_return_type_annotation(
            make::token(T![:]),
            TsAnyReturnType::TsType(TsType::TsVoidType(make::ts_void_type(make::token(T![
                void
            ])))),
        ))
        .build(),
    ));

    member_declarations.push(JsAnyObjectMember::JsMethodObjectMember(
        make::js_method_object_member(
            JsAnyObjectMemberName::JsLiteralMemberName(make::js_literal_member_name(make::ident(
                "destroy",
            ))),
            make::js_parameters(
                make::token(T!['(']),
                make::js_parameter_list(None, None),
                make::token(T![')']),
            ),
            make::js_function_body(
                make::token(T!['{']),
                make::js_directive_list(None),
                make::js_statement_list(Some(JsAnyStatement::JsExpressionStatement(
                    make::js_expression_statement(JsAnyExpression::JsCallExpression(
                        make::js_call_expression(
                            JsAnyExpression::JsStaticMemberExpression(
                                make::js_static_member_expression(
                                    JsAnyExpression::JsIdentifierExpression(
                                        make::js_identifier_expression(
                                            make::js_reference_identifier(make::ident("transport")),
                                        ),
                                    ),
                                    make::token(T![.]),
                                    JsAnyName::JsName(make::js_name(make::ident("destroy"))),
                                ),
                            ),
                            make::js_call_arguments(
                                make::token(T!['(']),
                                make::js_call_argument_list(None, None),
                                make::token(T![')']),
                            ),
                        )
                        .build(),
                    ))
                    .build(),
                ))),
                make::token(T!['}']),
            ),
        )
        .build(),
    ));

    items.push(JsAnyModuleItem::JsExport(make::js_export(
        make::token(T![export]),
        JsAnyExportClause::JsAnyDeclarationClause(JsAnyDeclarationClause::TsInterfaceDeclaration(
            make::ts_interface_declaration(
                make::token(T![interface]),
                make::ts_identifier_binding(make::ident("Workspace")),
                make::token(T!['{']),
                make::ts_type_member_list(member_definitions),
                make::token(T!['}']),
            )
            .build(),
        )),
    )));

    let member_separators = (0..member_declarations.len()).map(|_| make::token(T![,]));

    items.push(JsAnyModuleItem::JsExport(make::js_export(
        make::token(T![export]),
        JsAnyExportClause::JsAnyDeclarationClause(JsAnyDeclarationClause::JsFunctionDeclaration(
            make::js_function_declaration(
                make::token(T![function]),
                JsAnyBinding::JsIdentifierBinding(make::js_identifier_binding(make::ident(
                    "createWorkspace",
                ))),
                make::js_parameters(
                    make::token(T!['(']),
                    make::js_parameter_list(
                        Some(JsAnyParameter::JsAnyFormalParameter(
                            JsAnyFormalParameter::JsFormalParameter(
                                make::js_formal_parameter(JsAnyBindingPattern::JsAnyBinding(
                                    JsAnyBinding::JsIdentifierBinding(make::js_identifier_binding(
                                        make::ident("transport"),
                                    )),
                                ))
                                .with_type_annotation(make::ts_type_annotation(
                                    make::token(T![:]),
                                    TsType::TsReferenceType(
                                        make::ts_reference_type(TsAnyName::JsReferenceIdentifier(
                                            make::js_reference_identifier(make::ident("Transport")),
                                        ))
                                        .build(),
                                    ),
                                ))
                                .build(),
                            ),
                        )),
                        None,
                    ),
                    make::token(T![')']),
                ),
                make::js_function_body(
                    make::token(T!['{']),
                    make::js_directive_list(None),
                    make::js_statement_list(Some(JsAnyStatement::JsReturnStatement(
                        make::js_return_statement(make::token(T![return]))
                            .with_argument(JsAnyExpression::JsObjectExpression(
                                make::js_object_expression(
                                    make::token(T!['{']),
                                    make::js_object_member_list(
                                        member_declarations,
                                        member_separators,
                                    ),
                                    make::token(T!['}']),
                                ),
                            ))
                            .build(),
                    ))),
                    make::token(T!['}']),
                ),
            )
            .with_return_type_annotation(make::ts_return_type_annotation(
                make::token(T![:]),
                TsAnyReturnType::TsType(TsType::TsReferenceType(
                    make::ts_reference_type(TsAnyName::JsReferenceIdentifier(
                        make::js_reference_identifier(make::ident("Workspace")),
                    ))
                    .build(),
                )),
            ))
            .build(),
        )),
    )));

    let module = make::js_module(
        make::js_directive_list(None),
        make::js_module_item_list(items),
        make::eof(),
    )
    .build();

    let formatted = format_node(JsFormatOptions::default(), module.syntax()).unwrap();
    let printed = formatted.print();
    let code = printed.into_code();

    update(&bindings_path, &code, &mode)?;

    Ok(())
}
