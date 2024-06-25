<template>
    <div>
        <button @click="pobierzWpisy">Pobierz wpisy</button>
        siema blog!
        {{ wpisy }}
        <input v-model="nowyBlog" type="text">
        <button @click="dodajWpis">Dodaj wpis</button>
        {{ nowyBlog }}
    </div>
</template>

<script>
import { day2_backend } from 'declarations/day2_backend/index';

export default
{
    data()
    {
        return {
            wpisy: [],
            nowyBlog: ""
        }
    },
    methods:
    {
        async pobierzWpisy()
        {
            this.wpisy = await day2_backend.odczytaj_wpisy();
        },
        async dodajWpis()
        {
            await day2_backend.dodaj_wpis(this.nowyBlog);
        }
    },
}
</script>