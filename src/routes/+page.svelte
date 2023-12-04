<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import Chart from './Chart.svelte';
	import Matrix from './Matrix.svelte';
	import Scores from './Scores.svelte';
	import RegScores from './RegScores.svelte';

	let files: FileList;
	let decimalPoint = 2;
	let okClickPromise: Promise<{
		labels: Array<string> | undefined;
		datasets: {
			label: string;
			data: number[] | ROCPoint[];
		}[];
	}>;
	let okClick = false;
	// Do regresji
	let isReg = false;
	let MAE: Array<number> = [0, 0];
	let MAPE: Array<number> = [0, 0];
	let MSE: Array<number> = [0, 0];
	let RMSE: Array<number> = [0, 0];
	// Do klasyfikacji
	type ROCPoint = {
		x: number;
		y: number;
	};
	let err_matrix: Array<Array<number>>;
	let roc_data: Array<Array<ROCPoint>>;
	let auc: Array<number>;

	function handleOkClick() {
		okClick = true;
		okClickPromise = fetchAllData();
	}

	async function fetchAllData() {
		let a = await files[0].text();
		let headers: Array<String> = await invoke('parse_csv_headers', { unparsed: a });
		let parsedCSVArray: Array<Array<number>>;
		// regresja
		if (headers.length === 3) {
			isReg = true;
			parsedCSVArray = await invoke('parse_csv_reg', { unparsed: a });
			MAE = await invoke('mae', { unparsed: a });
			MAPE = await invoke('mape', { unparsed: a });
			MSE = await invoke('mse', { unparsed: a });
			RMSE = [Math.sqrt(MSE[0]), Math.sqrt(MSE[1])];
			return createBarChartData(headers, parsedCSVArray);
			// klasyfikacja
		} else if (headers.length === 5) {
			err_matrix = await invoke('error_matrix', { unparsed: a });
			roc_data = await invoke('roc_data', { unparsed: a });
			auc = await invoke('calculate_auc', { unparsed: a });
			return createROCChartData(roc_data);
		}
		return createBarChartData(headers, [[]]);
	}

	function createROCChartData(data: Array<Array<ROCPoint>>) {
		return {
			labels: undefined,
			datasets: [
				{
					label: 'Model 1',
					data: data[0],
					showLine: true
				},
				{
					label: 'Model 2',
					data: data[1],
					showLine: true
				}
			]
		};
	}

	function createBarChartData(
		headers: Array<String>,
		data: Array<Array<number>>
	): {
		labels: Array<string>;
		datasets: {
			label: string;
			data: number[];
		}[];
	} {
		return {
			// labels: xValues.map(e => e.toString()),
			labels: [...Array(data.length).keys()].map((i) => String(i + 1)),
			datasets: [
				{
					label: headers[0].toString(),
					data: data.map((r) => Number(r[0]))
				},
				{
					label: headers[1].toString(),
					data: data.map((r) => Number(r[1]))
				},
				{
					label: headers[2].toString(),
					data: data.map((r) => Number(r[2]))
				}
			]
		};
	}
</script>

{#if !okClick}
	<div class="hero min-h-screen bg-base-200">
		<div class="hero-content text-center">
			<div class="max-w-md">
				<h1 class="mb-5 text-5xl font-bold py-8">Zadanie programistyczne 2</h1>
				<p class="mb-5">Michał Gromniak 303066</p>
				<div class="form-control w-full max-w-xs">
					<!-- svelte-ignore a11y-label-has-associated-control -->
					<label class="label">
						<span class="label-text">Zaimportuj plik z danymi</span>
					</label>
					<input
						type="file"
						bind:files
						class="file-input file-input-bordered w-full max-w-xs"
						accept=".csv"
					/>

					{#if files && files.length != 0}
						<button class="btn btn-primary" on:click={handleOkClick}>Potwierdzam Wybór</button>
					{/if}
				</div>
			</div>
		</div>
	</div>
{:else}
	{#await okClickPromise}
		<div class="flex items-center justify-center h-screen">
			<h1 class="h1">Ładowanie...</h1>
			<span class="loading loading-dots loading-lg"></span>
		</div>
	{:then data}
		<div class="container p-8">
			{#if isReg}
				<div class="flex flex-row justify-between">
					<div class="flex flex-col">
						<RegScores {MAE} {MAPE} {MSE} {RMSE} first={true} {decimalPoint}></RegScores>
					</div>
					<div class="flex flex-col">
						<RegScores {MAE} {MAPE} {MSE} {RMSE} first={false} {decimalPoint}></RegScores>
					</div>
				</div>
				<h2>
					Uwaga wszystkie wartości są z zaokrągleniem do <input
						type="number"
						bind:value={decimalPoint}
						min="0"
						max="10"
						class="input input-bordered input-accent"
					/> miejsc po przecinku
				</h2>
			{:else}
				<div class="flex flex-row justify-between">
					<div class="flex flex-col">
						<h1 class="h1">Model 1 - Macierz pomyłek</h1>
						<Matrix err_matrix={err_matrix[0]}></Matrix>
						<Scores matrix1={err_matrix[0]} matrix2={err_matrix[1]} {decimalPoint}></Scores>
						<p>AUC = {auc[0].toFixed(decimalPoint)}</p>
					</div>
					<div class="flex flex-col">
						<h1 class="h1">Model 2 - Macierz pomyłek</h1>
						<Matrix err_matrix={err_matrix[1]}></Matrix>
						<Scores matrix1={err_matrix[1]} matrix2={err_matrix[0]} {decimalPoint}></Scores>
						<p>AUC = {auc[1].toFixed(decimalPoint)}</p>
					</div>
				</div>

				<h2>
					Uwaga wszystkie wartości są z zaokrągleniem do <input
						type="number"
						bind:value={decimalPoint}
						min="0"
						max="10"
						class="input input-bordered input-accent"
					/> miejsc po przecinku
				</h2>
			{/if}
			<div class="p-4 flex-auto">
				<Chart {data}></Chart>
			</div>
		</div>
	{/await}
{/if}
