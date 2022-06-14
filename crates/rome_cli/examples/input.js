tap.test(
	"RecordImport.advance", (t) => {
		mockFS(
			(callback) => {
				batch.setResults(
					[fs.createReadStream(dataFile)], (err) => {
						getBatches(
							(err, batches) => {
								checkStates(batches, ["started"]);

								RecordImport.advance(
									(err) => {
										t.error(err, "Error should be empty.");

										getBatches(
											(err, batches) => {
												checkStates(batches, ["process.completed"]);

												// Need to manually move to the next step
												batch.importRecords(
													(err) => {
														t.error(err, "Error should be empty.");

														getBatches(
															(err, batches) => {
																checkStates(batches, ["import.completed"]);

																RecordImport.advance(
																	(err) => {
																		t.error(err, "Error should be empty.");
																	},
																);

																t.ok(batch.getCurState().name(i18n));
															},
														);
													},
												);

												t.ok(batch.getCurState().name(i18n));
											},
										);
									},
								);

								t.ok(batch.getCurState().name(i18n));
							},
						);
					},
				);
			},
		);
	},
);
