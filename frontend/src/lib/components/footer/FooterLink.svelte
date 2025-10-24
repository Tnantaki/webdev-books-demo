<script module lang="ts">
   export interface FooterLink {
      href: string;
      title: string;
   }
</script>

<script lang="ts">
   const { href, title }: FooterLink = $props();
</script>

<a class="text-muted-light dark:text-muted-dark hover:text-primary" {href}>{title}</a>
