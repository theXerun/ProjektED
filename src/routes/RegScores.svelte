<script lang="ts">

    export let MAE: Array<number>
    export let MAPE: Array<number>
    export let MSE: Array<number>
    export let RMSE: Array<number>
    
    export let decimalPoint: number
    export let first = true;
    
    let n = first ? 0 : 1
    let m = first ? 1 : 0

</script>
<h1>Model {n+1}</h1>
<h2>MAE</h2>
<p class={(MAE[n]<MAE[m]) ? "better" : "worse"}>{MAE[n].toFixed(decimalPoint)}</p>
<h2>MAPE</h2>
<p class={(MAPE[n]<MAPE[m]) ? "better" : "worse"}>{MAPE[n].toFixed(decimalPoint)}%</p>
<h2>MSE</h2>
<p class={(MSE[n]<MSE[m]) ? "better" : "worse"}>{MSE[n].toFixed(decimalPoint)}</p>
<h2>RMSE</h2>
<p class={(RMSE[n]<RMSE[m]) ? "better" : "worse"}>{RMSE[n].toFixed(decimalPoint)}</p>


<style>
	.better {
		color: green;
	}
    .worse {
        color: red;
    }
</style>