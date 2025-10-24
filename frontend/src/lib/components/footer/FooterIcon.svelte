<script module lang="ts">
   export interface FooterIcon {
      href: string;
      title: string;
      icon_class: string;
   }
</script>

<script lang="ts">
   const { href, title, icon_class }: FooterIcon = $props();
</script>

<a
   class="text-muted-light dark:text-muted-dark hover:text-primary"
   {href}
   aria-label="link to {title}"
>
   <span class="size-6 {icon_class}"></span>
</a>
