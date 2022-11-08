import React, { useEffect, useState } from "react";
import { init, stage, elab } from "staged-comp";
import CodeMirror from "@uiw/react-codemirror";
import { atomone } from "@uiw/codemirror-theme-atomone";

import styles from "./index.module.scss";

export const App = function () {
	const [input, setInput] = useState(
`comp_let a: Nat1 = zero1 in
let b: Nat0 = zero0 in
comp_let c: Nat0 = zero0 in
let d: Nat0 = zero0 in
b`
	);
	const [err, setErr] = useState(false);
	const [output, setOutput] = useState("Output goes here...");
	const onChange = React.useCallback((value, _) => {
		setInput(value);
	}, []);
	useEffect(init, []);
	return <div className={ styles.container }>
			<div className={ styles.input }>
				<CodeMirror
					value={ input }
					onChange={onChange}
					theme={ atomone }
				/>
			</div>
			<div className={ styles.output }>
				<div className={ styles.buttons }>
					<button onClick={ () => {
						try {
							const e = elab(input);
							setOutput(e);
							setErr(false);
						} catch {
							setOutput("Errored");
							setErr(true);
						}
					} }>Elaborate</button>
					<button onClick={ () => {
						try {
							const e = stage(input);
							setOutput(e);
							setErr(false);
						} catch {
							setOutput("Errored");
							setErr(true);
						}
					} }>Stage</button>
				</div>
				<div className={ styles.result } data-err={ err.toString() }>
					{ output }
				</div>
			</div>
	</div>;
}
