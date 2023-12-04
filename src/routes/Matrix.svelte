<script lang="ts">
    export let err_matrix: Array<number>
</script>

<div class="overflow-x-auto">
    <table class="table">
        <!-- head -->
        <thead>
            <tr>
                <th></th>
                <th>0</th>
                <th>1</th>
            </tr>
        </thead>
        <tbody>
            <!-- row 1 -->
            <tr>
                <th>0</th>
                <td>{err_matrix[0]}</td>
                <td>{err_matrix[1]}</td>
            </tr>
            <!-- row 2 -->
            <tr>
                <th>1</th>
                <td>{err_matrix[2]}</td>
                <td>{err_matrix[3]}</td>
            </tr>
        </tbody>
    </table>
</div>