import React, { useEffect, useState } from "react";
import { init, stage, elab } from "staged-comp";
import CodeMirror from "@uiw/react-codemirror";
import { atomone } from "@uiw/codemirror-theme-atomone";

import styles from "./index.module.scss";

export const App = function () {
	const [input, setInput] = useState(
`const nat1_to_nat0
: (_: Nat1) -> Nat0 =
nat_elim1 (^Nat0) zero0 ((m: ^Nat0) => <succ0 m>)
in

const add1
: (_: Nat1) -> (_: Nat1) -> Nat1
= (a: Nat1) => nat_elim1 Nat1 a succ1
in

const a: Nat1 = succ1 (succ1 zero1) in
const b: Nat1 = succ1 zero1 in

let c: Nat0 = ~(nat1_to_nat0 (add1 a b)) in
c
`
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
						} catch (err) {
							setOutput(`${ err }`);
							setErr(true);
						}
					} }>Elaborate</button>
					<button onClick={ () => {
						try {
							const e = stage(input);
							setOutput(e);
							setErr(false);
						} catch (err) {
							setOutput(`${ err }`);
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
