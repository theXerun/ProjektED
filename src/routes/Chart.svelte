<script lang="ts">
	import { onMount } from 'svelte';
	import Chart, { type ChartItem } from 'chart.js/auto';
	import ZoomPlugin from 'chartjs-plugin-zoom';
	Chart.register(ZoomPlugin);

	type ROCPoint = {
		x: number;
		y: number;
	};

	export let data: {
		labels: Array<string> | undefined;
		datasets: {
			label: string;
			data: number[] | ROCPoint[];
		}[];
	};

	let ctx: ChartItem;
	let chart: Chart;

	$: if (chart) {
		chart.data = data;
		chart.update();
	}

	onMount(async () => {
		if (typeof data.datasets[0].data[0] === 'number') {
			chart = new Chart(ctx, {
				type: 'bar',
				data: data,
				options: {
					plugins: {
						zoom: {
							pan: {
								enabled: true
							},
							zoom: {
								wheel: {
									enabled: true
								},
								pinch: {
									enabled: true
								},
								mode: 'xy'
							}
						},
						tooltip: {
							enabled: false
						}
					}
				}
			});
		} else {
			chart = new Chart(ctx, {
				type: 'scatter',
				data: data,
				options: {
					aspectRatio: 1,
					plugins: {
						tooltip: {
							enabled: false
						}
					}
				}
			});
		}
	});
</script>

<div>
	<canvas id="chart" bind:this={ctx} />
</div>
