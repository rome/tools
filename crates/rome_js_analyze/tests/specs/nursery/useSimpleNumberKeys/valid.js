/* should not generate diagnostics */

({ name: 1, monkey: 2 });

({ 0: "zero" });
({ 1: "one" });
({ 1.2: "12" });
({ 3.1e12: "12" });
({ .1e12: "ee" });
